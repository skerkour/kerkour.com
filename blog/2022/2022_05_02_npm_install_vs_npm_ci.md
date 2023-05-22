+++
date = 2022-05-02T02:00:00Z
title = "NPM install VS NPM ci"
type = "post"
tags = ["programming", "javascript", "tutorial"]
authors = ["Sylvain Kerkour"]
url = "/npm-install-vs-npm-ci"

[extra]
lang = "en"

comment ="""
"""
+++

<!--
https://nodejs.dev/learn/the-package-lock-json-file
https://docs.npmjs.com/cli/v6/configuring-npm/package-locks
https://docs.npmjs.com/cli/v8/commands/npm-ci
https://docs.npmjs.com/cli/v8/commands/npm-install
https://blog.npmjs.org/post/171556855892/introducing-npm-ci-for-faster-more-reliable
 -->


**TL;DR:** Use `npm ci` in your CI/CD pipelines and `npm install` on your machine.

When running `npm install`, you may notice that the command modifies your `package-lock.json` file from time to time.

[There are many reasons for that](https://docs.npmjs.com/cli/v6/configuring-npm/package-locks):
* different versions of npm (or other package managers) may have been used to install a package, each using slightly different installation algorithms.
* a new version of a direct semver-range package may have been published since the last time your packages were installed, and thus a newer version will be used.
* A dependency of one of your dependencies may have published a new version, which will update even if you used pinned dependency specifiers (1.2.3 instead of ^1.2.3)
* The registry you installed from is no longer available, or allows mutation of versions (unlike the primary npm registry), and a different version of a package exists under the same version number now.

Thus, if you are using [`npm install`](https://docs.npmjs.com/cli/v8/commands/npm-install) in your CI/CD pipeline, it may make your builds not reproducible.


This is why [in 2018](https://blog.npmjs.org/post/171556855892/introducing-npm-ci-for-faster-more-reliable), NPM introduced the [`npm ci`](https://docs.npmjs.com/cli/v8/commands/npm-ci) command to install the exact same package as declared in your `package-lock.json` file and make your builds faster and reproducible.
