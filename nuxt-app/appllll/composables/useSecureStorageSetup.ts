import { ref } from "vue";
import type { Client } from "@tauri-apps/plugin-stronghold";
import { Stronghold } from "@tauri-apps/plugin-stronghold";
import { appDataDir } from "@tauri-apps/api/path";

const VAULT_FILE = "vault.hold";
const CLIENT_NAME = "main";

let strongholdInstance: Stronghold | null = null;
let clientInstance: Client | null = null;

export function useStronghold() {
  const initialized = ref(false);

  const init = async (password: string) => {
    if (initialized.value && strongholdInstance && clientInstance) {
      return;
    }

    const vaultPath = `${await appDataDir()}${VAULT_FILE}`;

    const stronghold = await Stronghold.load(vaultPath, password);

    let client: Client;

    try {
      client = await stronghold.loadClient(CLIENT_NAME);
    } catch {
      client = await stronghold.createClient(CLIENT_NAME);
    }

    strongholdInstance = stronghold;
    clientInstance = client;

    initialized.value = true;
  };

  const getStore = () => {
    if (!clientInstance) {
      throw new Error("Stronghold not initialized");
    }

    return clientInstance.getStore();
  };

  const setItem = async (key: string, value: string | object) => {
    const store = getStore();

    const encoded = Array.from(new TextEncoder().encode(JSON.stringify(value)));

    await store.insert(key, encoded);

    await strongholdInstance?.save();
  };

  const getItem = async <T>(key: string): Promise<T | null> => {
    const store = getStore();

    const data = await store.get(key);

    if (!data) {
      return null;
    }

    const decoded = new TextDecoder().decode(new Uint8Array(data));

    return JSON.parse(decoded);
  };

  const removeItem = async (key: string) => {
    const store = getStore();

    await store.remove(key);

    await strongholdInstance?.save();
  };

  return {
    initialized,
    init,
    setItem,
    getItem,
    removeItem,
  };
}
