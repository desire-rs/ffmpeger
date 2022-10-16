#!/bin/bash
rm -rf dist
rm -rf .parcel-cache
parcel build src/index.pug --public-url ./assets --no-source-maps --no-cache