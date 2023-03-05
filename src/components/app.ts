import { convertFileSrc } from "@tauri-apps/api/tauri";
import * as bulmaToast from "bulma-toast";
import classnames from "classnames";
import m from "mithril";

import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { Event as TauriEvent, listen, UnlistenFn } from "@tauri-apps/api/event";
import { AppActions, AppState, Config, WheelState } from "../types";
import Settings from "./settings";
import Wheel from "./wheel";
import errorToast from "../func";

const EMPTY_NAME = "-----";

export default class App {
  private actions: AppActions;

  private audioDevices: [string, string][] = [];

  private background?: m.Child = null;

  private currentList: string = "";

  private lists: string[] = [];

  private isRefreshingLists: boolean = false;

  private state: AppState;

  private unlistens: UnlistenFn[] = [];

  private wheelItems: string[] = [
    EMPTY_NAME,
    EMPTY_NAME,
    EMPTY_NAME,
    EMPTY_NAME,
    EMPTY_NAME,
  ];

  private wheelState: WheelState = WheelState.Stopped;

  constructor() {
    this.actions = this.appActions();
    this.state = {
      config: {
        audioDevice: "",
        music: true,
        reverseChance: 0.25,
        speedReduceMax: 0.05,
        speedReduceMin: 0.03,
        speedSlowLimit: 1.0,
        speedSlowReduceMax: 0.001,
        speedSlowReduceMin: 0.01,
        speedStartMax: 5.0,
        speedStartMin: 4.5,
        speedStopMax: 0.5,
        speedStopMin: 0.25,
        startFullscreen: false,
        volume: 100,
      },
      settingsActive: false,
    };

    bulmaToast.setDefaults({
      animate: { in: "fadeIn", out: "fadeOut" },
      dismissible: true,
      position: "top-center",
    });
  }

  async oninit() {
    document.addEventListener(
      "contextmenu",
      (e) => {
        e.preventDefault();
      },
      false
    );

    document.addEventListener("keydown", (e: KeyboardEvent) =>
      this.onKeyDown(e)
    );

    this.unlistens.push(
      await listen("wheel-list", (event: TauriEvent<string[]>) => {
        this.wheelItems = event.payload;
        m.redraw();
      })
    );

    this.unlistens.push(
      await listen("stop", () => {
        this.wheelState = WheelState.Stopped;
        m.redraw();
      })
    );

    await this.refreshLists();
    if (this.lists.length > 0) {
      [this.currentList] = this.lists;
    }

    this.getBackground();
  }

  view() {
    return [
      this.background,
      m(
        "section#content.section",
        m(
          ".container.has-text-centered",
          m(
            ".block",
            m(
              ".select.is-fullwidth",
              {
                class: classnames({
                  "is-loading": this.isRefreshingLists,
                }),
              },
              m(
                "select",
                {
                  disabled: this.wheelState !== WheelState.Stopped,
                  onchange: (e: Event) => this.onListChanged(e),
                },
                this.lists.map((listName) => m("option", listName))
              )
            )
          ),
          m(
            ".block",
            m(
              "button.button.is-large",
              {
                class: classnames({
                  "is-primary": this.wheelState !== WheelState.Rolling,
                  "is-danger": this.wheelState === WheelState.Rolling,
                }),
                disabled:
                  this.lists.length === 0 ||
                  this.wheelState === WheelState.Stopping,
                onclick: () => this.onRollClicked(),
              },
              this.wheelState === WheelState.Rolling ? "Stop" : "Roll"
            )
          ),
          m(Wheel, {
            wheelItems: this.wheelItems,
          })
        )
      ),
      m(Settings, {
        actions: this.actions,
        audioDevices: this.audioDevices,
        state: this.state,
      }),
    ];
  }

  private getBackground() {
    invoke<[string, string]>("random_bg", { listName: this.currentList })
      .then((data: [string, string]) => {
        if (data === null) {
          this.background = null;
        } else if (data[1].startsWith("image/")) {
          this.background = m("img#bg", {
            src: convertFileSrc(data[0], "data"),
          });
        } else if (data[1].startsWith("video/")) {
          this.background = m(
            "video#bg[autoplay][loop][muted]",
            {
              onupdate: ({ dom }) => (dom as HTMLVideoElement).load(),
            },
            [
              m("source", {
                src: convertFileSrc(data[0], "data"),
                type: data[1],
              }),
            ]
          );
        }
        m.redraw();
      })
      .catch((error) => errorToast(error));
  }

  private appActions(): AppActions {
    return {
      setSettingsActive: (value: boolean) => {
        if (this.state.settingsActive === value) {
          return;
        }

        if (value) {
          invoke<Config>("get_config").then((config: Config) => {
            this.state.config = config;
            invoke<[string, string][]>("get_audio_devices")
              .then((devices: [string, string][]) => {
                this.audioDevices = devices;
                this.state.settingsActive = value;
                m.redraw();
              })
              .catch((error) => errorToast(error));
          });
        } else {
          this.state.settingsActive = value;
        }
      },
    };
  }

  private async onKeyDown(event: KeyboardEvent) {
    switch (event.code) {
      case "Escape":
        this.actions.setSettingsActive(false);
        m.redraw();
        break;
      case "F1":
        if (this.wheelState === WheelState.Stopped) {
          this.actions.setSettingsActive(true);
          m.redraw();
        }
        break;
      case "F5":
        event.preventDefault();
        break;
      case "F11":
        await appWindow.setFullscreen(!(await appWindow.isFullscreen()));
        break;
      default:
        break;
    }
  }

  private onListChanged(event: Event) {
    this.currentList = (event.target as HTMLSelectElement).value;
    this.getBackground();
  }

  private onRollClicked() {
    switch (this.wheelState) {
      case WheelState.Stopped:
        this.wheelState = WheelState.Rolling;
        invoke("roll", { listName: this.currentList }).catch((error) => {
          errorToast(error);
          this.wheelState = WheelState.Stopped;
        });
        break;
      case WheelState.Rolling:
        this.wheelState = WheelState.Stopping;
        invoke("stop")
          .then(() => {})
          .catch((error) => {
            errorToast(error);
            this.wheelState = WheelState.Rolling;
          });
        break;
      default:
        break;
    }
  }

  private async refreshLists() {
    this.toggleListRefreshing(true);
    this.lists = await invoke("lists");
    this.toggleListRefreshing(false);
  }

  private toggleListRefreshing(value: boolean) {
    this.isRefreshingLists = value;
    m.redraw();
  }
}
