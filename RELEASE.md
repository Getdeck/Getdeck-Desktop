# Releases

## How to release a new version of Getdeck Desktop

1. Make sure you have `jq` and `toml-cli` installed and available in your path
2. From the project root, run `./bumpversion.sh $VERSION_NUMBER` and commit the result
3. Create a new tag with the version number that you are publishing
4. Push the tag and watch the CI/CD pipeline build bundles and create a new release on Github
5. Once the pipeline finishes (make sure every OS build went through smoothly and all files are present),
    run `./rollout.sh $VERSION_NUMBER` to adapt `updater.json`.
6. Commit the result, Tauri Updater will now notify users that new version is ready to download.
