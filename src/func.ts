import * as bulmaToast from "bulma-toast";

export default function errorToast(message: string) {
  bulmaToast.toast({
    message,
    duration: 5000,
    type: "is-danger",
  });
}
