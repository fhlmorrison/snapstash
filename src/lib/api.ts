import { invoke, convertFileSrc as tauriConvertFileSrc } from "@tauri-apps/api/core";

export const isTauri = !!(window as any).__TAURI_INTERNALS__;
const API_BASE_URL = "http://127.0.0.1:8080";

export function convertFileSrc(path: string): string {
    if (isTauri) {
        return tauriConvertFileSrc(path);
    } else {
        return `${API_BASE_URL}/file/${encodeURIComponent(path)}`;
    }
}

export async function apiReadDir(src: string): Promise<any[]> {
    if (isTauri) {
        // This is tricky because readDir is from @tauri-apps/plugin-fs
        // and we can't easily import it here without side effects if we want to mock it.
        // But we'll assume the caller handles the choice if they want, 
        // or we can provide a unified interface.
        throw new Error("apiReadDir should only be called in non-tauri mode or unified");
    } else {
        const response = await fetch(`${API_BASE_URL}/read_dir`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ src }),
        });
        if (!response.ok) {
            const text = await response.text();
            throw new Error(text || response.statusText);
        }
        return await response.json();
    }
}

export async function apiInvoke<T>(command: string, args: Record<string, any> = {}): Promise<T> {
  if (isTauri) {
    return await invoke<T>(command, args);
  } else {
    const response = await fetch(`${API_BASE_URL}/${command}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(args),
    });
    
    if (!response.ok) {
        const text = await response.text();
        throw new Error(text || response.statusText);
    }

    if (response.status === 204 || response.headers.get("content-length") === "0") {
        return undefined as any;
    }

    return await response.json();
  }
}

// For GET requests like get_tags
export async function apiGet<T>(command: string): Promise<T> {
    if (isTauri) {
        return await invoke<T>(command);
    } else {
        const response = await fetch(`${API_BASE_URL}/${command}`);
        if (!response.ok) {
            const text = await response.text();
            throw new Error(text || response.statusText);
        }
        return await response.json();
    }
}
