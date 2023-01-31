# How to run this example?

## 1. Develop and Build

You can start developing your template by running build in watch mode.

```sh
pacd build --watch --data-path ./data.json --output-path ./build ./site
```

Running the command above will start watching the `./site` directory for any changes and build files into the `./build`
directory.

## 2. Package and Distribute

After you are done developig your templates, stop the build command if it is running. Run the `pack` command
to create a `tar.gz` file of your template files.

```sh
pacd pack --output-path build.tar.gz ./site
```

You can distrbute this archive to anyone who has `pacd` to build their own sites in the template you developed. They
can also supply a different data to the build command.

## 3. Build sites from archives

Build sites from packaged archives with your own data. Assuming you have a ecommerce_template.tar.gz, you can run the following
command to build your site.

```sh
pacd build --data-path ./inventory.json --output-path ./build ./ecommerce_template.tar.gz
```
