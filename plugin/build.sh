#!/bin/bash
rm -rf dist
tsc src/scripts/*.ts --outDir dist/
parcel build src/index.pug
cp -R images dist/images
cp manifest.json dist/manifest.json