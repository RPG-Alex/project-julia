<a name="readme-top"></a>

# Project Julia

<div align="center">
    <img src="concept.png" alt="Concept Julia Fractal" width="360" height="360">
</div>

<details>
    <summary>Contents</summary>
    <ol>
        <li>
            <a href="#purpose">Purpose</a>
        </li>
        <li>
            <a href="#contributing">Contributing</a>
        </li>
        <li>
            <a href="#incomplete-features">Incomplete Features</a>
        </li>
        <li>
            <a href="#installation-and-usage">Installation and Usage</a>
        </li>
    </ol>
</details>

## Purpose

The goal of Project Julia is to dynamically render Julia sets in the browser using WebAssembly, showcasing the application of Rust for web development.

## Contributing

To contribute, simply fork this repository, make your modifications, and then submit a pull request that clearly details the changes and explanations.

## Incomplete Features

- [ ] Render a Julia set in the browser.
- [ ] Allow for adjustable inputs to change the output Julia set.
- [ ] Ability to zoom into the set.
    - [ ] Infinite zoom.
 - [ ] Export generated Julia sets as images.
 - [ ] Implement different color schemes for rendered Julia sets.
 - [ ] Add user preferences and settings to customize the rendering.
 - [ ] Implement keyboard shortcuts for common actions like zooming, panning, and resetting.
 - [ ] Create a gallery to view and compare multiple Julia sets simultaneously.
 - [ ] Add the ability to share rendered Julia sets on social media platforms.
 - [ ] Implement touch and gesture support for mobile devices.
 - [ ] Provide tooltips and information overlays explaining the mathematics behind Julia sets.

## Installation and Usage

### How to Install

```sh
npm install
```
How to Run
Debug Mode

Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.

```sh
npm start
```
Release Mode

Builds the project and places it into the dist folder.

```sh
npm run build
```
Unit Tests
Run tests in Firefox
```sh
npm test -- --firefox
```
Run tests in Chrome
```sh
npm test -- --chrome
```
Run tests in Safari
```sh
npm test -- --safari
```
Project Structure

   - Cargo.toml contains the standard Rust metadata for dependencies.
   - package.json contains the standard npm metadata for JavaScript dependencies.
   - webpack.config.js contains the Webpack configuration.
   - The js folder contains JavaScript code.
   - The src folder contains Rust code.
   - The static folder contains files to be copied into the final build, including an index.html file.
   - The tests folder contains Rust unit tests.

<p align="center">[<a href="#readme-top">RETURN TO TOP</a>]</p>
