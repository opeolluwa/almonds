import { defineStore } from "pinia";
import { HttpLink } from "@apollo/client/link/http";

type BackupProvider = "local" | "cloud" | "self-hosted";

interface BackupServerConfig {
  provider: BackupProvider;
  selfHostedApiUrl: string;
  selfHostedApiKey: string;
}

const STRONGHOLD_KEY = "sync-server";

export const useBackupSettingsStore = defineStore("backup_settings", {
  state: () => ({
    initialized: false,
    provider: "local" as BackupProvider,
    selfHostedApiUrl: "",
    selfHostedApiKey: "",
    savedConfig: null as BackupServerConfig | null,
  }),

  getters: {
    savedConfigExists: (state) => state.savedConfig !== null,
  },

  actions: {
    async init() {
      if (this.initialized) return;

      const stronghold = useStronghold();

      try {
        await stronghold.init(STRONGHOLD_KEY);
        this.initialized = true;
      } catch (error) {
        console.error("Failed to initialize stronghold:", error);
        return;
      }

      try {
        const config =
          await stronghold.getItem<BackupServerConfig>(STRONGHOLD_KEY);
        if (config) {
          this.savedConfig = config;
          this.provider = config.provider;
          this.selfHostedApiUrl = config.selfHostedApiUrl;
          this.selfHostedApiKey = config.selfHostedApiKey;
          this.applyEndpoint();
        }
      } catch (error) {
        console.error("Failed to load backup config:", error);
      }
    },

    applyEndpoint() {
      const { client } = useApolloClient();
      if (this.provider === "self-hosted" && this.selfHostedApiUrl) {
        client.setLink(new HttpLink({ uri: this.selfHostedApiUrl }));
      }
    },

    async save() {
      const stronghold = useStronghold();

      const payload: BackupServerConfig = {
        provider: this.provider,
        selfHostedApiUrl: this.selfHostedApiUrl,
        selfHostedApiKey: this.selfHostedApiKey,
      };

      await stronghold.setItem(STRONGHOLD_KEY, payload);
      this.savedConfig = payload;
      this.applyEndpoint();
    },
  },
});
