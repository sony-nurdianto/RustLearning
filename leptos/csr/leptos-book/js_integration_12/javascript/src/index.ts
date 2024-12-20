import { DotLottie } from "@lottiefiles/dotlottie-web";
// import animationData from "../test_asset/animations/7cfd6934-56db-4135-adbc-2649bf1d461d.json";

export function buildCanvas(lottie_path: string, canvas: HTMLCanvasElement) {
  // let canvas = document.getElementById("canv") as HTMLCanvasElement;
  new DotLottie({
    autoplay: true,
    loop: true,
    canvas: canvas,
    data: JSON.parse(lottie_path),
  });
}
