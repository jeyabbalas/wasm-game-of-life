# Conway's Game of Life in Wasm

This repository follows the tutorial from [Rust Wasm book's](https://rustwasm.github.io/docs/book/) that introduces Wasm via Rust.


## Initial Setup

1. The initial project setup was done by cloning the [wasm-pack project template](https://github.com/rustwasm/rust-webpack-template) and specifying the project name using the following command—

```sh
> cargo generate --git https://github.com/rustwasm/wasm-pack-template --name wasm-game-of-life
```

2. To build the project (Rust --> Wasm), the command below was run. It creates a `pkg/` folder containing the `*.wasm` binary and the JavaScript glue code (`*.js`). It also contains the TypeScript type declarations file for TypeScript users (`*.d.ts`).

```sh
> wasm-pack build
```

3. To create a web app to run the wasm module, another project template called [create-wasm-app](https://github.com/rustwasm/create-wasm-app) was cloned via [NPM](https://www.npmjs.com/package/create-wasm-app). The command below copies the template into a new folder `www/`.

```sh
> npm init wasm-app www
```

4. The dependency file `www/package.json` was customized to the current project.

```json
// www/package.json
{
  // ...
  "author": "Jeya Balaji Balasubramanian <jeyabbalas@gmail.com>",
  "dependencies": {
    "wasm-game-of-life": "file:../pkg"
  },
  "devDependencies": {
    //...
  }
}
```

5. The `www/index.js` file was also customized to import the package in `pkg/`.

```js
import * as wasm from "wasm-game-of-life";

wasm.greet();
```

6. The new npm package is installed within the `www/` folder by running the following command. It generates a `/www/node_modules/` folder.

```sh
> npm install
```

7. A server is run to serve the webpage within `/www/`.

```sh
> npm run start
```

This generated the `www/index.html` content at http://localhost:8080/.

After making updates to the Rust code, simply re-run `> wasm-pack build` to automatically reflect changes in the served page.
