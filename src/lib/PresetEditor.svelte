<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { Profile, ProfileOptions } from "../types";
  import { defaultProfileOptions } from "../types";

  export let profile: Profile | null;

  const dispatch = createEventDispatcher<{
    save: Profile;
    cancel: void;
  }>();

  const isNew = profile === null;

  let name = profile?.name ?? "";
  let options: ProfileOptions = profile
    ? { ...profile.options }
    : defaultProfileOptions();

  // fullscreen の三値を select で扱うために変換
  let fullscreenValue: "null" | "true" | "false" =
    options.fullscreen === true
      ? "true"
      : options.fullscreen === false
        ? "false"
        : "null";

  $: options.fullscreen =
    fullscreenValue === "true"
      ? true
      : fullscreenValue === "false"
        ? false
        : null;

  let nameError = "";

  function save() {
    if (!name.trim()) {
      nameError = "名前を入力してください";
      return;
    }
    nameError = "";
    const saved: Profile = {
      id: profile?.id ?? crypto.randomUUID(),
      name: name.trim(),
      options: { ...options },
    };
    dispatch("save", saved);
  }
</script>

<div class="container">
  <header>
    <button class="btn-back" on:click={() => dispatch("cancel")}>← 戻る</button>
    <h1>{isNew ? "新規プリセット" : "プリセット編集"}</h1>
  </header>

  <div class="form-scroll">
    <!-- プリセット名 -->
    <section class="section">
      <label class="section-label" for="preset-name">プリセット名</label>
      <input
        id="preset-name"
        type="text"
        placeholder="例: 普段用"
        bind:value={name}
        on:input={() => (nameError = "")}
      />
      {#if nameError}
        <span class="field-error">{nameError}</span>
      {/if}
    </section>

    <!-- VR / デバッグ -->
    <section class="section">
      <div class="section-title">VR / デバッグ</div>
      <div class="options-grid">
        <label class="opt-row">
          <input type="checkbox" bind:checked={options.no_vr} />
          <span class="opt-label">
            <span class="opt-name">No VR モード</span>
            <span class="opt-flag">--no-vr</span>
          </span>
        </label>
        <label class="opt-row">
          <input type="checkbox" bind:checked={options.enable_debug_gui} />
          <span class="opt-label">
            <span class="opt-name">Debug GUI</span>
            <span class="opt-flag">--enable-debug-gui</span>
          </span>
        </label>
        <label class="opt-row">
          <input
            type="checkbox"
            bind:checked={options.enable_udon_debug_logging}
          />
          <span class="opt-label">
            <span class="opt-name">Udon Debug ログ</span>
            <span class="opt-flag">--enable-udon-debug-logging</span>
          </span>
        </label>
        <label class="opt-row">
          <input
            type="checkbox"
            bind:checked={options.enable_sdk_log_levels}
          />
          <span class="opt-label">
            <span class="opt-name">SDK ログレベル</span>
            <span class="opt-flag">--enable-sdk-log-levels</span>
          </span>
        </label>
        <label class="opt-row">
          <input
            type="checkbox"
            bind:checked={options.disable_hw_video_decoding}
          />
          <span class="opt-label">
            <span class="opt-name">HW ビデオデコード無効</span>
            <span class="opt-flag">--disable-hw-video-decoding</span>
          </span>
        </label>
      </div>
    </section>

    <!-- グラフィック -->
    <section class="section">
      <div class="section-title">グラフィック</div>
      <div class="options-grid">
        <label class="opt-row">
          <input type="checkbox" bind:checked={options.d3d11} />
          <span class="opt-label">
            <span class="opt-name">DirectX 11 強制</span>
            <span class="opt-flag">-force-d3d11</span>
          </span>
        </label>
        <label class="opt-row">
          <input type="checkbox" bind:checked={options.popupwindow} />
          <span class="opt-label">
            <span class="opt-name">ポップアップウィンドウ</span>
            <span class="opt-flag">-popupwindow</span>
          </span>
        </label>
      </div>
    </section>

    <!-- 数値オプション -->
    <section class="section">
      <div class="section-title">パフォーマンス / 画面</div>
      <div class="numeric-grid">
        <div class="num-row">
          <label for="fps-input" class="num-label">
            FPS 上限
            <span class="opt-flag">--fps</span>
          </label>
          <div class="num-control">
            <input
              id="fps-input"
              type="number"
              min="1"
              max="360"
              placeholder="無制限"
              value={options.fps ?? ""}
              on:input={(e) => {
                const v = (e.target as HTMLInputElement).value;
                options.fps = v === "" ? null : parseInt(v, 10);
              }}
            />
          </div>
        </div>

        <div class="num-row">
          <label for="width-input" class="num-label">
            解像度 幅
            <span class="opt-flag">-screen-width</span>
          </label>
          <div class="num-control">
            <input
              id="width-input"
              type="number"
              min="320"
              placeholder="デフォルト"
              value={options.screen_width ?? ""}
              on:input={(e) => {
                const v = (e.target as HTMLInputElement).value;
                options.screen_width = v === "" ? null : parseInt(v, 10);
              }}
            />
          </div>
        </div>

        <div class="num-row">
          <label for="height-input" class="num-label">
            解像度 高さ
            <span class="opt-flag">-screen-height</span>
          </label>
          <div class="num-control">
            <input
              id="height-input"
              type="number"
              min="240"
              placeholder="デフォルト"
              value={options.screen_height ?? ""}
              on:input={(e) => {
                const v = (e.target as HTMLInputElement).value;
                options.screen_height = v === "" ? null : parseInt(v, 10);
              }}
            />
          </div>
        </div>

        <div class="num-row">
          <label for="priority-input" class="num-label">
            プロセス優先度
            <span class="opt-flag">-2 ~ 2</span>
          </label>
          <div class="num-control">
            <input
              id="priority-input"
              type="number"
              min="-2"
              max="2"
              placeholder="通常 (0)"
              value={options.process_priority ?? ""}
              on:input={(e) => {
                const v = (e.target as HTMLInputElement).value;
                options.process_priority = v === "" ? null : parseInt(v, 10);
              }}
            />
            <span class="priority-hint">低← →高</span>
          </div>
        </div>

        <div class="num-row">
          <label for="fullscreen-select" class="num-label">
            表示モード
            <span class="opt-flag">-fullscreen / -windowed</span>
          </label>
          <div class="num-control">
            <select id="fullscreen-select" bind:value={fullscreenValue}>
              <option value="null">デフォルト</option>
              <option value="true">フルスクリーン</option>
              <option value="false">ウィンドウ</option>
            </select>
          </div>
        </div>
      </div>
    </section>

    <!-- カスタムオプション -->
    <section class="section">
      <label class="section-label" for="custom-input">
        カスタムオプション
        <span class="opt-flag">（スペース区切り）</span>
      </label>
      <input
        id="custom-input"
        type="text"
        placeholder="例: --extra-option --another"
        bind:value={options.custom}
      />
    </section>
  </div>

  <div class="footer">
    <button class="btn-cancel" on:click={() => dispatch("cancel")}>
      キャンセル
    </button>
    <button class="btn-save" on:click={save}>保存</button>
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

  .form-scroll {
    flex: 1;
    overflow-y: auto;
    padding: 14px 16px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .section-label {
    font-size: 11px;
    color: #888;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .section-title {
    font-size: 11px;
    color: #666;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border-bottom: 1px solid #1e1e35;
    padding-bottom: 4px;
  }

  .field-error {
    color: #e94560;
    font-size: 11px;
  }

  /* チェックボックスグリッド */
  .options-grid {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .opt-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 7px 10px;
    border-radius: 4px;
    cursor: pointer;
    transition: background 0.1s;
  }

  .opt-row:hover {
    background: #1a1a35;
  }

  .opt-label {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
  }

  .opt-name {
    font-size: 13px;
    color: #ddd;
  }

  .opt-flag {
    font-size: 11px;
    color: #555;
    font-family: "Consolas", monospace;
  }

  /* 数値入力グリッド */
  .numeric-grid {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .num-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 6px 10px;
    border-radius: 4px;
  }

  .num-row:hover {
    background: #1a1a35;
  }

  .num-label {
    flex: 1;
    font-size: 13px;
    color: #ddd;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .num-control {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 160px;
    flex-shrink: 0;
  }

  .num-control input,
  .num-control select {
    width: 100%;
    background: #0d0d1f;
    border: 1px solid #2a2a4a;
    border-radius: 4px;
    color: #e0e0e0;
    padding: 5px 8px;
    font-size: 12px;
  }

  .num-control select {
    cursor: pointer;
  }

  .priority-hint {
    font-size: 10px;
    color: #444;
    white-space: nowrap;
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
