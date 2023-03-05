import { invoke } from "@tauri-apps/api";
import classnames from "classnames";
import m from "mithril";

import { AppActions, AppState } from "../types";
import errorToast from "../func";

interface SettingsAttrs {
  actions: AppActions;
  audioDevices: [string, string][];
  state: AppState;
}

function formControl(...children: m.Children[]): m.Children {
  return m(".field", m(".control", children));
}

function formField(
  label: string,
  labelClasses: string,
  ...children: m.Children[]
): m.Children {
  return m(".field.is-horizontal", [
    m(".field-label", { class: labelClasses }, m("label.label", label)),
    m(".field-body", children),
  ]);
}

function randomRange(
  label: string,
  minName: string,
  maxName: string,
  min: number,
  max: number
): m.Children {
  return formField(
    label,
    "is-normal",
    m(
      ".field",
      m(
        "p.control.is-expanded",
        m("input.input[type=number][min=0]", {
          name: minName,
          placeholder: "Minimum",
          value: min,
        })
      )
    ),
    m(
      ".field",
      m(
        "p.control.is-expanded",
        m("input.input[type=number][min=0]", {
          name: maxName,
          placeholder: "Maximum",
          value: max,
        })
      )
    )
  );
}

export default class Settings {
  private actions: AppActions;

  private state: AppState;

  constructor(vnode: m.Vnode<SettingsAttrs>) {
    this.actions = vnode.attrs.actions;
    this.state = vnode.attrs.state;
  }

  view(vnode: m.Vnode<SettingsAttrs>) {
    return m(
      ".modal",
      { class: classnames({ "is-active": this.state.settingsActive }) },

      m(".modal-background", {
        onclick: (e: Event) => this.closeModal(e.target),
      }),
      m(
        ".modal-card",
        m("header.modal-card-head", [
          m("p.modal-card-title", "Settings"),
          m("button.delete[aria-label=close]", {
            onclick: (e: Event) => this.closeModal(e.target),
          }),
        ]),
        m(
          "section.modal-card-body",
          {
            onchange: (e: Event) => this.onFormFieldChanged(e),
          },
          [
            formField(
              "Start in full screen",
              "",
              formControl(
                m("input[name=startFullscreen][type=checkbox]", {
                  checked: this.state.config.startFullscreen,
                })
              )
            ),
            formField(
              "Music",
              "",
              formControl(
                m("input[name=music][type=checkbox]", {
                  checked: this.state.config.music,
                })
              )
            ),
            formField(
              "Audio device",
              "",
              formControl(
                m(
                  ".select",
                  m(
                    "select.is-fullwidth[name=audioDevice]",
                    vnode.attrs.audioDevices.map((data) =>
                      m(
                        "option",
                        {
                          selected: data[0] === this.state.config.audioDevice,
                          value: data[0],
                        },
                        data[1]
                      )
                    )
                  )
                )
              )
            ),
            formField(
              "Volume",
              "is-normal",
              formControl(
                m(
                  "input#volume.slider.is-fullwidth.has-output[name=volume][type=range][min=0][max=100][step=1]",
                  {
                    value: this.state.config.volume,
                  }
                ),
                m("output", { for: "volume" }, this.state.config.volume)
              )
            ),
            formField(
              "Reverse roll chance",
              "is-normal",
              formControl(
                m(
                  "input#reverse-chance.slider.is-fullwidth.has-output[name=reverseChance][data-float][type=range][min=0][max=1][step=0.01]",
                  {
                    value: this.state.config.reverseChance,
                  }
                ),
                m(
                  "output",
                  { for: "reverse-chance" },
                  this.state.config.reverseChance
                )
              )
            ),
            randomRange(
              "Starting speed",
              "speedStartMin",
              "speedStartMax",
              this.state.config.speedStartMin,
              this.state.config.speedStartMax
            ),
            randomRange(
              "Speed reduction",
              "speedReduceMin",
              "speedReduceMax",
              this.state.config.speedReduceMin,
              this.state.config.speedReduceMax
            ),
            formField(
              'Switch to "slow" at',
              "is-normal",
              formControl(
                m("input.input[name=speedSlowLimit][type=number]", {
                  value: 1.0,
                })
              )
            ),
            randomRange(
              "Slow reduction",
              "speedSlowReduceMin",
              "speedSlowReduceMax",
              this.state.config.speedSlowReduceMin,
              this.state.config.speedSlowReduceMax
            ),
            randomRange(
              'Speed after "stop" press',
              "speedStopMin",
              "speedStopMax",
              this.state.config.speedStopMin,
              this.state.config.speedStopMax
            ),
          ]
        ),
        m("footer.modal-card-foot", [
          m(
            "button.button.is-success",
            {
              onclick: (e: Event) => this.onSaveClicked(e),
            },
            "Save"
          ),
          m(
            "button.button",
            {
              onclick: (e: Event) => this.closeModal(e.target),
            },
            "Cancel"
          ),
        ])
      )
    );
  }

  private closeModal(element: EventTarget | null) {
    (element as Element | null)
      ?.closest(".modal")
      ?.classList.remove("is-active");
    this.actions.setSettingsActive(false);
  }

  private onFormFieldChanged(event: Event) {
    if (event.target === null) {
      return;
    }

    const target = event.target as HTMLInputElement;
    let value;
    switch (target.type) {
      case "range":
        value =
          target.dataset.float === undefined
            ? parseInt(target.value, 10)
            : parseFloat(target.value);
        break;
      case "checkbox":
        value = target.checked;
        break;
      case "number":
        value = parseFloat(target.value);
        break;
      default:
        value = target.value;
        break;
    }
    this.state.config[target.name] = value;
  }

  private onSaveClicked(e: Event) {
    invoke("set_config", { config: this.state.config })
      .then(() => {
        this.closeModal(e.target);
      })
      .catch((error) => errorToast(error));
  }
}
