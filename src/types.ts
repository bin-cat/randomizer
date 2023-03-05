export enum WheelState {
  Stopped,
  Rolling,
  Stopping,
}

export interface AppActions {
  setSettingsActive(value: boolean): void;
}

export interface AppState {
  config: Config;
  settingsActive: boolean;
}

export interface Config {
  audioDevice: string;
  music: boolean;
  reverseChance: number;
  speedReduceMax: number;
  speedReduceMin: number;
  speedSlowLimit: number;
  speedSlowReduceMax: number;
  speedSlowReduceMin: number;
  speedStartMax: number;
  speedStartMin: number;
  speedStopMax: number;
  speedStopMin: number;
  startFullscreen: boolean;
  volume: number;
  [key: string]: string | number | boolean;
}
