<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  export let vrchatPath: string;

  const dispatch = createEventDispatcher<{
    save: string;
    back: void;
  }>();

  let pathInput = vrchatPath;
  let detecting = false;
  let detectError = "";
  let detectSuccess = "";

  async function detectPath() {
    detecting = true;
    detectError = "";
    detectSuccess = "";
    try {
      const detected = await invoke<string>("detect_vrchat_path");
      pathInput = detected;
      detectSuccess = "VRChat.exe を検出しました";
    } catch (e) {
      detectError = `検出失敗: ${e}`;
    } finally {
      detecting = false;
    }
  }

  function save() {
    if (!pathInput.trim()) return;
    dispatch("save", pathInput.trim());
  }
</script>

<div class="container">
  <header>
    {#if vrchatPath}
      <button class="btn-back" on:click={() => dispatch("back")}>← 戻る</button>
    {/if}
    <h1>設定</h1>
  </header>

  <div class="content">
    <section class="section">
      <label class="section-title" for="path-input">VRChat 実行ファイルのパス</label>

      <div class="path-row">
        <input
          id="path-input"
          type="text"
          placeholder="C:\...\VRChat.exe"
          bind:value={pathInput}
          on:input={() => {
            detectError = "";
            detectSuccess = "";
          }}
        />
      </div>

      <button
        class="btn-detect"
        on:click={detectPath}
        disabled={detecting}
      >
        {detecting ? "検索中..." : "🔍 Steam から自動検出"}
      </button>

      {#if detectSuccess}
        <div class="msg-success">{detectSuccess}</div>
      {/if}
      {#if detectError}
        <div class="msg-error">{detectError}</div>
      {/if}

      <p class="hint">
        Steam がインストールされている場合は自動検出をお試しください。<br />
        手動でパスを指定することもできます。
      </p>
    </section>
  </div>

  <div class="footer">
    {#if vrchatPath}
      <button class="btn-cancel" on:click={() => dispatch("back")}>
        キャンセル
      </button>
    {/if}
    <button class="btn-save" on:click={save} disabled={!pathInput.trim()}>
      保存
    </button>
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  header {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background: #0d0d1f;
    border-bottom: 1px solid #222;
    flex-shrink: 0;
  }

  h1 {
    font-size: 14px;
    font-weight: 600;
    color: #c0b0ff;
  }

  .btn-back {
    background: transparent;
    color: #888;
    font-size: 12px;
    padding: 4px 8px;
  }

  .content {
    flex: 1;
    padding: 24px 20px;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 10px;
    max-width: 480px;
  }

  .section-title {
    font-size: 12px;
    color: #888;
    margin-bottom: 2px;
  }

  .path-row {
    display: flex;
    gap: 8px;
  }

  .path-row input {
    flex: 1;
  }

  .btn-detect {
    background: #1e3a5f;
    color: #7eb8f7;
    align-self: flex-start;
    padding: 7px 14px;
    font-size: 13px;
  }

  .msg-success {
    color: #4caf50;
    font-size: 12px;
    padding: 6px 10px;
    background: #0a2a0a;
    border: 1px solid #1a5c1a;
    border-radius: 4px;
  }

  .msg-error {
    color: #e94560;
    font-size: 12px;
    padding: 6px 10px;
    background: #2a0a0a;
    border: 1px solid #5c1a1a;
    border-radius: 4px;
  }

  .hint {
    font-size: 11px;
    color: #555;
    line-height: 1.6;
    margin-top: 4px;
  }

  .footer {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    padding: 12px 16px;
    background: #0d0d1f;
    border-top: 1px solid #222;
    flex-shrink: 0;
  }

  .btn-cancel {
    background: #2a2a4a;
    color: #aaa;
    padding: 8px 18px;
  }

  .btn-save {
    background: #6e44ff;
    color: #fff;
    font-weight: 600;
    padding: 8px 22px;
  }
</style>
