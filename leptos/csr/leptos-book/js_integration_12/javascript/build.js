try {
  await Bun.build({
    entrypoints: ["./src/index.ts"],
    outdir: "../javascript_external",
    target: "bun",
    minify: {
      whitespace: true,
      identifiers: true,
      syntax: true,
    },
  });
} catch (error) {
  throw new Error(error);
}
