<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { Config, Profile } from "./types";
  import PresetList from "./lib/PresetList.svelte";
  import PresetEditor from "./lib/PresetEditor.svelte";
  import Settings from "./lib/Settings.svelte";

  type View = "presets" | "editor" | "settings";

  let config: Config | null = $state(null);
  let currentView: View = $state("presets");
  let editingProfile: Profile | null = $state(null);
  let loading = $state(true);
  let initError = $state("");

  onMount(async () => {
    try {
      config = await invoke<Config>("get_config");
      if (!config.vrchat_path) {
        currentView = "settings";
      }
    } catch (e) {
      initError = String(e);
    } finally {
      loading = false;
    }
  });

  async function handleSaveConfig(newConfig: Config) {
    try {
      await invoke("save_config", { config: newConfig });
      config = newConfig;
    } catch (e) {
      alert(`保存に失敗しました: ${e}`);
    }
  }

  function navigateTo(view: View, profile: Profile | null = null) {
    editingProfile = profile;
    currentView = view;
  }

  async function handleSavePreset(savedProfile: Profile) {
    if (!config) return;
    const idx = config.profiles.findIndex((p) => p.id === savedProfile.id);
    const newProfiles =
      idx >= 0
        ? config.profiles.map((p) => (p.id === savedProfile.id ? savedProfile : p))
        : [...config.profiles, savedProfile];
    await handleSaveConfig({ ...config, profiles: newProfiles });
    navigateTo("presets");
  }

  async function handleDeletePreset(id: string) {
    if (!config) return;
    await handleSaveConfig({
      ...config,
      profiles: config.profiles.filter((p) => p.id !== id),
    });
  }

  async function handleSaveSettings(path: string) {
    if (!config) return;
    await handleSaveConfig({ ...config, vrchat_path: path });
    navigateTo("presets");
  }
</script>

<main>
  {#if loading}
    <div class="loading">起動中...</div>
  {:else if initError}
    <div class="error-screen">
      <p>初期化エラー: {initError}</p>
    </div>
  {:else if config}
    {#if currentView === "presets"}
      <PresetList
        {config}
        onedit={(p) => navigateTo("editor", p)}
        onnew={() => navigateTo("editor", null)}
        ondelete={handleDeletePreset}
        onsettings={() => navigateTo("settings")}
      />
    {:else if currentView === "editor"}
      <PresetEditor
        profile={editingProfile}
        onsave={handleSavePreset}
        oncancel={() => navigateTo("presets")}
      />
    {:else if currentView === "settings"}
      <Settings
        vrchatPath={config.vrchat_path}
        onsave={handleSaveSettings}
        onback={() => navigateTo("presets")}
      />
    {/if}
  {/if}
</main>

<style>
  :global(*) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(body) {
    background: #1a1a2e;
    color: #e0e0e0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    font-size: 14px;
    overflow: hidden;
    user-select: none;
  }

  :global(button) {
    cursor: pointer;
    border: none;
    border-radius: 4px;
    padding: 6px 14px;
    font-size: 13px;
    font-weight: 500;
    transition: filter 0.15s;
  }

  :global(button:hover:not(:disabled)) {
    filter: brightness(1.15);
  }

  :global(button:disabled) {
    opacity: 0.45;
    cursor: not-allowed;
  }

  :global(input[type="text"]),
  :global(input[type="number"]) {
    background: #0d0d1f;
    border: 1px solid #333;
    border-radius: 4px;
    color: #e0e0e0;
    padding: 6px 10px;
    font-size: 13px;
    width: 100%;
  }

  :global(input[type="text"]:focus),
  :global(input[type="number"]:focus) {
    outline: 1px solid #6e44ff;
    border-color: #6e44ff;
  }

  :global(input[type="checkbox"]) {
    width: 15px;
    height: 15px;
    accent-color: #6e44ff;
    cursor: pointer;
  }

  main {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #888;
    font-size: 16px;
  }

  .error-screen {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #e94560;
    padding: 24px;
    text-align: center;
  }
</style>
