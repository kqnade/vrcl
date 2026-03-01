<script lang="ts">
  import { untrack } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { Config, Profile } from "../types";

  interface Props {
    config: Config;
    onedit: (profile: Profile) => void;
    onnew: () => void;
    ondelete: (id: string) => void;
    onsettings: () => void;
  }

  let { config, onedit, onnew, ondelete, onsettings }: Props = $props();

  let selectedId: string | null = $state(
    untrack(() => (config.profiles.length > 0 ? config.profiles[0].id : null))
  );
  let launching = $state(false);
  let launchError = $state("");
  let deletingId: string | null = $state(null);

  let selectedProfile = $derived(
    config.profiles.find((p) => p.id === selectedId) ?? null
  );

  async function launchVrchat() {
    if (!selectedProfile) return;
    launching = true;
    launchError = "";
    try {
      await invoke("launch_vrchat", {
        vrchat_path: config.vrchat_path,
        profile: selectedProfile,
      });
    } catch (e) {
      launchError = String(e);
    } finally {
      launching = false;
    }
  }

  function executeDelete(id: string) {
    deletingId = null;
    if (selectedId === id) {
      const remaining = config.profiles.filter((p) => p.id !== id);
      selectedId = remaining.length > 0 ? remaining[0].id : null;
    }
    ondelete(id);
  }
</script>

<div class="container">
  <header>
    <h1>VRChat Launch Manager</h1>
    <button class="btn-settings" onclick={onsettings}>⚙ 設定</button>
  </header>

  <div class="content">
    <div class="sidebar">
      <div class="sidebar-header">
        <span class="label">プリセット</span>
        <button class="btn-new" onclick={onnew}>＋ 新規</button>
      </div>

      {#if config.profiles.length === 0}
        <div class="empty">
          プリセットがありません。<br />「＋ 新規」で作成してください。
        </div>
      {:else}
        <ul class="preset-list">
          {#each config.profiles as profile (profile.id)}
            <li
              class="preset-item"
              class:selected={selectedId === profile.id}
              onclick={() => (selectedId = profile.id)}
              onkeydown={(e) => e.key === "Enter" && (selectedId = profile.id)}
              role="option"
              aria-selected={selectedId === profile.id}
              tabindex="0"
            >
              <span class="preset-name">{profile.name}</span>
            </li>
          {/each}
        </ul>
      {/if}
    </div>

    <div class="detail">
      {#if selectedProfile}
        <div class="detail-header">
          <h2>{selectedProfile.name}</h2>
          <div class="detail-actions">
            <button class="btn-edit" onclick={() => onedit(selectedProfile!)}>
              編集
            </button>
            <button class="btn-delete" onclick={() => (deletingId = selectedProfile!.id)}>
              削除
            </button>
          </div>
        </div>

        <div class="options-summary">
          <h3>オプション一覧</h3>
          <ul>
            {#if selectedProfile.options.no_vr}
              <li><code>--no-vr</code></li>
            {/if}
            {#if selectedProfile.options.enable_debug_gui}
              <li><code>--enable-debug-gui</code></li>
            {/if}
            {#if selectedProfile.options.enable_udon_debug_logging}
              <li><code>--enable-udon-debug-logging</code></li>
            {/if}
            {#if selectedProfile.options.enable_sdk_log_levels}
              <li><code>--enable-sdk-log-levels</code></li>
            {/if}
            {#if selectedProfile.options.disable_hw_video_decoding}
              <li><code>--disable-hw-video-decoding</code></li>
            {/if}
            {#if selectedProfile.options.fps !== null}
              <li><code>--fps={selectedProfile.options.fps}</code></li>
            {/if}
            {#if selectedProfile.options.process_priority !== null}
              <li><code>プロセス優先度: {selectedProfile.options.process_priority}</code></li>
            {/if}
            {#if selectedProfile.options.screen_width !== null}
              <li><code>-screen-width={selectedProfile.options.screen_width}</code></li>
            {/if}
            {#if selectedProfile.options.screen_height !== null}
              <li><code>-screen-height={selectedProfile.options.screen_height}</code></li>
            {/if}
            {#if selectedProfile.options.fullscreen === true}
              <li><code>-fullscreen</code></li>
            {:else if selectedProfile.options.fullscreen === false}
              <li><code>-windowed</code></li>
            {/if}
            {#if selectedProfile.options.d3d11}
              <li><code>-force-d3d11</code></li>
            {/if}
            {#if selectedProfile.options.popupwindow}
              <li><code>-popupwindow</code></li>
            {/if}
            {#if selectedProfile.options.custom}
              <li><code>{selectedProfile.options.custom}</code></li>
            {/if}
            {#if !Object.values(selectedProfile.options).some((v) => v !== false && v !== null && v !== "")}
              <li class="no-options">（オプションなし）</li>
            {/if}
          </ul>
        </div>

        {#if launchError}
          <div class="launch-error">{launchError}</div>
        {/if}

        <button class="btn-launch" onclick={launchVrchat} disabled={launching}>
          {launching ? "起動中..." : "▶ VRChat 起動"}
        </button>
      {:else}
        <div class="no-selection">プリセットを選択してください</div>
      {/if}
    </div>
  </div>
</div>

<!-- 削除確認ダイアログ -->
{#if deletingId}
  {@const id = deletingId}
  <div class="overlay" role="presentation" onclick={() => (deletingId = null)}>
    <div
      class="dialog"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <p>このプリセットを削除しますか？</p>
      <div class="dialog-actions">
        <button class="btn-cancel" onclick={() => (deletingId = null)}>キャンセル</button>
        <button class="btn-delete" onclick={() => executeDelete(id)}>削除</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .container {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    background: #0d0d1f;
    border-bottom: 1px solid #222;
  }

  h1 {
    font-size: 15px;
    font-weight: 600;
    color: #c0b0ff;
  }

  .btn-settings {
    background: transparent;
    color: #888;
    font-size: 12px;
    padding: 4px 8px;
  }

  .content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .sidebar {
    width: 200px;
    background: #12122a;
    border-right: 1px solid #222;
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    border-bottom: 1px solid #222;
  }

  .label {
    font-size: 11px;
    color: #666;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .btn-new {
    background: #6e44ff;
    color: #fff;
    font-size: 11px;
    padding: 4px 8px;
  }

  .empty {
    padding: 20px 12px;
    color: #555;
    font-size: 12px;
    line-height: 1.6;
    text-align: center;
  }

  .preset-list {
    list-style: none;
    overflow-y: auto;
    flex: 1;
  }

  .preset-item {
    padding: 10px 14px;
    cursor: pointer;
    border-bottom: 1px solid #1a1a35;
    transition: background 0.1s;
    outline: none;
  }

  .preset-item:hover {
    background: #1e1e40;
  }

  .preset-item.selected {
    background: #2a1a55;
    border-left: 3px solid #6e44ff;
  }

  .preset-name {
    font-size: 13px;
    color: #ddd;
  }

  .detail {
    flex: 1;
    padding: 16px;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }

  .detail-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .detail-header h2 {
    font-size: 16px;
    font-weight: 600;
    color: #e0e0e0;
  }

  .detail-actions {
    display: flex;
    gap: 8px;
  }

  .btn-edit {
    background: #1e3a5f;
    color: #7eb8f7;
    font-size: 12px;
    padding: 5px 10px;
  }

  .btn-delete {
    background: #3d1a1a;
    color: #e94560;
    font-size: 12px;
    padding: 5px 10px;
  }

  .options-summary {
    flex: 1;
    margin-bottom: 16px;
  }

  .options-summary h3 {
    font-size: 11px;
    color: #666;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 10px;
  }

  .options-summary ul {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .options-summary li code {
    background: #0d0d22;
    border: 1px solid #2a2a4a;
    border-radius: 3px;
    padding: 3px 8px;
    font-family: "Consolas", "Courier New", monospace;
    font-size: 12px;
    color: #a0d8ef;
  }

  .no-options {
    color: #555;
    font-size: 12px;
    font-style: italic;
  }

  .launch-error {
    background: #2d0a0a;
    border: 1px solid #e94560;
    border-radius: 4px;
    color: #e94560;
    font-size: 12px;
    padding: 8px 12px;
    margin-bottom: 12px;
  }

  .btn-launch {
    background: #6e44ff;
    color: #fff;
    font-size: 15px;
    font-weight: 600;
    padding: 12px 24px;
    border-radius: 6px;
    letter-spacing: 0.03em;
    align-self: flex-start;
  }

  .no-selection {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
    color: #555;
    font-size: 14px;
  }

  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .dialog {
    background: #1e1e3a;
    border: 1px solid #333;
    border-radius: 8px;
    padding: 24px;
    min-width: 260px;
    text-align: center;
  }

  .dialog p {
    margin-bottom: 20px;
    font-size: 14px;
    color: #ccc;
  }

  .dialog-actions {
    display: flex;
    gap: 10px;
    justify-content: center;
  }

  .btn-cancel {
    background: #2a2a4a;
    color: #ccc;
  }
</style>
