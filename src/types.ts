export interface ProfileOptions {
  no_vr: boolean
  enable_debug_gui: boolean
  enable_udon_debug_logging: boolean
  enable_sdk_log_levels: boolean
  disable_hw_video_decoding: boolean
  fps: number | null
  process_priority: number | null  // -2 ~ 2
  screen_width: number | null
  screen_height: number | null
  fullscreen: boolean | null
  d3d11: boolean
  popupwindow: boolean
  custom: string
}

export interface Profile {
  id: string
  name: string
  options: ProfileOptions
}

export interface Config {
  vrchat_path: string
  profiles: Profile[]
}

export function defaultProfileOptions(): ProfileOptions {
  return {
    no_vr: false,
    enable_debug_gui: false,
    enable_udon_debug_logging: false,
    enable_sdk_log_levels: false,
    disable_hw_video_decoding: false,
    fps: null,
    process_priority: null,
    screen_width: null,
    screen_height: null,
    fullscreen: null,
    d3d11: false,
    popupwindow: false,
    custom: "",
  }
}
