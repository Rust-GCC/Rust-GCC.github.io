# Rust-GCC.github.io

github pages for the Rust GCC project

## Running the website locally

The easiest way is to use the jekyll docker image if you are not familiar with jekyll and bundle.

```shell
docker run --rm -it -e UID=$(id -u) -e GID=$(id -g) -p 4000:4000 -v $(pwd):/srv/jekyll jekyll/builder bash -c 'bundle install && bundle exec jekyll serve'
```

Otherwise you can run locally:

```shell
$ bundle install
$ bundle exec jekyll serve
```
