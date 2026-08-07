import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  // Without this Next walks up and picks a stray lockfile in the parent
  // directory as the workspace root, which mistraces bundled files.
  outputFileTracingRoot: __dirname,
  webpack(config) {
    // The ezpdf wasm modules are loaded with `await init(url)` at runtime, but
    // webpack still needs to know how to emit the .wasm asset.
    config.experiments = { ...config.experiments, asyncWebAssembly: true };
    return config;
  },
  async headers() {
    return [
      {
        source: '/:path*.wasm',
        headers: [{ key: 'Content-Type', value: 'application/wasm' }],
      },
    ];
  },
};

export default nextConfig;
