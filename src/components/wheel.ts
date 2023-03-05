import m from "mithril";
import { writeText } from "@tauri-apps/api/clipboard";

const WHEEL_CLASSES = [
  "is-size-5 py-1",
  "is-size-4 py-2",
  "is-size-3 has-text-weight-bold py-4",
  "is-size-4 py-2",
  "is-size-5 py-1",
];

interface WheelAttrs {
  wheelItems: string[];
}

export default {
  view(vnode: m.Vnode<WheelAttrs>) {
    return m(
      ".block",
      m(
        "#wheel-items.box",
        vnode.attrs.wheelItems.map((item, index) =>
          m(
            "p.is-clickable",
            {
              class: WHEEL_CLASSES[index],
              onclick: async (e: Event) => {
                await writeText((e.target as HTMLParagraphElement).innerText);
              },
            },
            item
          )
        )
      )
    );
  },
};
