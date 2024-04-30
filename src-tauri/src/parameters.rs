// struct params {
//     params: Vec<String>,
//     negative: Vec<String>,
// }

const PARAM_KEYS: [&str; 12] = [
    "Negative prompt",
    "Steps",
    "Sampler",
    "CFG scale",
    "Seed",
    "Face restoration",
    "Size",
    "Model hash",
    "Clip skip",
    "ENSD",
    "Lora hashes",
    "Version",
];

pub fn get_prompts<'a>(params_string: &'a str) -> Vec<&'a str> {
    params_string
        .split(" Negative prompt: ")
        .next()
        .unwrap_or("")
        .split(",")
        .map(clean_prompt_token)
        .collect()
}

fn clean_prompt_token<'a>(s: &'a str) -> &'a str {
    s.trim()
        .trim_start_matches("(")
        .trim_end_matches(")")
        .trim_start_matches("[")
        .trim_end_matches("]")
}

#[cfg(test)]
mod params_test {
    use super::*;

    #[test]
    fn test_params() {
        let test_string = r#"giraffe, music, coloredic0n icon <lora:Colored_Icons:1> Negative prompt: EasyNegative, (((duplicate))), ((morbid)), ((mutilated)), [out of frame], extra fingers, mutated hands, ((poorly drawn hands)), ((poorly drawn face)), (((mutation))), (((deformed))), ((ugly)), blurry, ((bad anatomy)), (((bad proportions))), ((extra limbs)), cloned face, (((disfigured))), out of frame, ugly, extra limbs, (bad anatomy), gross proportions, (malformed limbs), ((missing arms)), ((missing legs)), (((extra arms))), (((extra legs))), mutated hands, (fused fingers), (too many fingers), (((long neck))), ugly, tiling, poorly drawn hands, poorly drawn feet, poorly drawn face, out of frame, mutation, mutated, extra limbs, extra legs, extra arms, disfigured, deformed, cross-eye, body out of frame, blurry, bad art, bad anatomy, extra ears, pointy ears, elf, elf ears, goat ears, Steps: 25, Sampler: Euler a, CFG scale: 7, Seed: 1804880831, Size: 512x512, Model hash: 8a952cafe9, ENSD: 31337, Lora hashes: "Colored_Icons: 1c97ad42e515", Version: v1.4.0"#;
        let parsed = get_prompts(test_string);
        // println!("{:?}", parsed);
        assert_eq!(
            parsed,
            vec![
                "giraffe",
                "music",
                "coloredic0n icon <lora:Colored_Icons:1>"
            ]
        );
    }
}
