use anyhow::Result;
use candle_core::{Device, Tensor, DType};
use candle_nn::VarBuilder;
use candle_transformers::models::siglip;
use hf_hub::api::sync::ApiBuilder;
use tokenizers::Tokenizer;
use std::sync::Mutex;
use std::path::Path;
use log::{info, warn};

pub struct ClipState {
    model: siglip::Model,
    tokenizer: Tokenizer,
    device: Device,
    config: siglip::Config,
}

impl ClipState {
    pub fn new() -> Result<Self> {
        println!("Determining device for SigLIP...");
        let device = if candle_core::utils::cuda_is_available() {
            println!("CUDA device found.");
            Device::new_cuda(0)?
        } else if candle_core::utils::metal_is_available() {
            println!("Metal device found.");
            Device::new_metal(0)?
        } else {
            println!("Hardware acceleration not available, using CPU.");
            Device::Cpu
        };

        // Using the base patch16-224 model as default, similar to the example
        let hf_repo = "google/siglip-base-patch16-224";
        println!("Connecting to Hugging Face Hub for SigLIP ({})", hf_repo);
        let api = ApiBuilder::new().with_progress(true).build()?;
        let repo = api.model(hf_repo.to_string());
        
        println!("Fetching SigLIP model, config and tokenizer...");
        let model_file = repo.get("model.safetensors")?;
        let config_file = repo.get("config.json")?;
        let tokenizer_file = repo.get("tokenizer.json")?;

        let config: siglip::Config = serde_json::from_slice(&std::fs::read(config_file)?)?;
        
        println!("Loading SigLIP model weights onto device...");
        let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_file], DType::F32, &device)? };
        let model = siglip::Model::new(&config, vb)?;
        
        println!("Loading tokenizer...");
        let tokenizer = Tokenizer::from_file(tokenizer_file).map_err(anyhow::Error::msg)?;

        println!("SigLIP state initialized successfully.");
        Ok(Self {
            model,
            tokenizer,
            device,
            config,
        })
    }

    pub fn get_text_embeddings(&self, prompts: &[String]) -> Result<Tensor> {
        let pad_id = self.config.text_config.pad_token_id;
        let max_len = self.config.text_config.max_position_embeddings;
        
        let mut tokens = vec![];
        for seq in prompts {
            let encoding = self.tokenizer.encode(seq.as_str(), true).map_err(anyhow::Error::msg)?;
            let mut ids = encoding.get_ids().to_vec();
            // Pad or truncate to max_position_embeddings as per SigLIP requirement
            let len_diff = max_len - ids.len();
            if len_diff > 0 {
                ids.extend(vec![pad_id; len_diff]);
            } else {
                ids.truncate(max_len);
            }
            tokens.push(ids);
        }

        let input_ids = Tensor::new(tokens, &self.device)?;
        let embeddings = self.model.get_text_features(&input_ids)?;
        
        // Normalize embeddings for similarity matching (dot product) in main.rs
        let embeddings = embeddings.broadcast_div(&embeddings.sqr()?.sum_keepdim(1)?.sqrt()?)?;
        Ok(embeddings)
    }

    pub fn get_image_embeddings(&self, path: &Path) -> Result<Tensor> {
        let image_size = self.config.vision_config.image_size;
        
        // Load and preprocess image as per SigLIP requirements
        let img = image::io::Reader::open(path)?
            .decode()?
            .resize_to_fill(image_size as u32, image_size as u32, image::imageops::FilterType::Triangle);
        
        let img = img.to_rgb8();
        let data = img.into_raw();
        
        // SigLIP normalization: affine(2. / 255., -1.) maps [0, 255] to [-1, 1]
        let data = Tensor::from_vec(data, (image_size, image_size, 3), &Device::Cpu)?
            .permute((2, 0, 1))?
            .to_dtype(DType::F32)?
            .affine(2. / 255., -1.)?
            .to_device(&self.device)?;
        
        let input = data.unsqueeze(0)?;
        let embeddings = self.model.get_image_features(&input)?;
        
        // Normalize embeddings
        let embeddings = embeddings.broadcast_div(&embeddings.sqr()?.sum_keepdim(1)?.sqrt()?)?;

        // Debug: Print the first few elements of the embedding
        if let Ok(vec) = embeddings.flatten_all()?.to_vec1::<f32>() {
            println!("Image embedding (first 10): {:?}", &vec[..10.min(vec.len())]);
        }

        Ok(embeddings)
    }
}

pub struct ClipStore {
    pub state: Mutex<Option<ClipState>>,
}

impl ClipStore {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(None),
        }
    }

    pub fn get_or_init(&self) -> Result<std::sync::MutexGuard<'_, Option<ClipState>>> {
        let mut state = self.state.lock().unwrap();
        if state.is_none() {
            println!("SigLIP store initializing for the first time...");
            *state = Some(ClipState::new()?);
        }
        Ok(state)
    }
}
