# CI/CDとは

CI/CDの例です｡CI(continuous integration)は継続的インテグレーションのことをいう｡これは､コードをマージしてもソフトウェアがビルドできることと意図した通りに動くことが検証されていることをいう｡CD(continuous delivery)はソフトウェアがリリースできる状態に維持されることをいう｡

これらはソフトウェアの自動化の流れで生まれたもの｡CIはソフトウェアのビルドやテストを自動化して頻繁に実行すること｡CDはCIに加えてデプロイまで自動化する手法のこと｡これらはGithubへのプッシュやマージをトリガーとして行われるため､Github Actionsという機能としてGithubが自ら実装した｡

# 構成

GitHubとGitHub Actionを利用する｡

# GitHub Actions

リポジトリに対するプッシュやプルリクエストといった操作や時刻をトリガーとしてあらかじめ定義しておいた処理を実行する機能｡

# ActionとWorkflow

GitHub Actionsでは実行する処理とその処理を実行する条件を定義したものを｢Workflow｣と呼ぶ｡YAMLで書く｡.github/workflowディレクトリに配置する｡

ワークフロー内ではシェル経由で任意のコマンドを実行できるほか､｢Action｣というあらかじめ定義済みの処理を呼び出すことができる｡Actionsの設定はリポジトリのActionsタブから行う｡

ワークフローの定義例を以下に示す｡

```
name: Rust <- ワークフロー名

on: [push] <- リポジトリへのpush時に実行

jobs: <- ここで実行するジョブを指定する
    build: <- buildという名前のジョブを定義

        runs-on: ubuntu-latest <- Ubuntuの最新バージョンで実行

        steps: <- 実行するコマンドや処理を指定
        - uses: actions/checkout@v2 <- リポジトリからチェックアウトを行う｢actions/checkout｣アクションを実行する
        - name: Build
          run: cargo build --verbose <- ｢cargo build｣コマンドを実行してビルドを行う
        - name: Run tests
          run: cargo test --verbose <- ｢cargo test｣コマンドを実行してテストを行う
```

ワークフローの書式について説明する

- on

ワークフローを起動するトリガーとなるイベントまたは時刻を記述する場所｡｢on: push｣とするとpushが行われたときにワークフローを起動する｡
作成と削除をトリガーとする場合は｢on: [create, delete]｣とする｡特定のブランチや特定のブランチ名へのpush時にのみ起動するようにする場合､以下のように書く｡

```
on:
  push:
    branches:
      - master
      - foo*
```

この場合､masterかfooから始まるブランチへpushがあったときにワークフローが起動する｡いくつかのイベントはタイプを持つものがある｡

```
on:
  issues:
    types: [opened, reopened]
```

この場合､issueが開いたとき､または再度開かれたときにワークフローが起動する｡スケジュールで起動する場合は以下のようにscheduleイベントを利用して書く｡

```
on:
  schedule:
    - cron: '30 * * * *'
```

cronでは｢<分> <時> <日> <月> <曜日>｣のようにして日時を指定できる｡上記の例は30分毎にワークフローを実行する｡

- runs-on

どの環境で実行するかを指定する｡指定できるものは以下の表の通り｡

|使用できる環境|
|----|
|windows-latest|
|ubuntu-latest|
|ubuntu-18.04|
|ubuntu-16.04|
|macos-latest|
|self-hosted|

- env

環境変数を指定できる｡書き方は以下の通り｡

```
name: <ワークフロー名>
env:
  <変数名1>: <値>
  <変数名2>: <値>
  ....
```

- jobs

処理内容を記述する場所｡ジョブにはジョブIDと名前､他のジョブとの依存関係を書く必要がある｡

```
jobs:
  foo:
    name: job foo
  bar:
    name: job bar
    needs: foo
```

実際に行う処理はstepsの下に書く｡

```
jobs:
  <ジョブID1>:
    name: <ジョブ名>
    steps:
      - name: <ステップ1の名前>
        run: <実行する処理>
        :
        :

      - name: <ステップ2の名前>
        run: <実行する処理>
        shell: bash
        :
        :
```

shellを使うことでシェルを指定することができる｡

- uses

useで指定したアクションを実行することができる｡

- with

withはuseで指定したアクションに与える引数を表す｡

- ワークフローの例

```
name: Build Deb Package
on: [push]
jobs:
  build:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v1

    - name: Build the Docker image
      run: docker build . --file Dockerfile --tag pylucene_build

    - name: Build .deb package
      run: docker run -v $(pwd):/host:rw pylucene_build

    - name: Copy artifact
      run: mkdir -p deb && cp -a python-lucene* pylucene_* deb/
    
    - name: actions/upload-srtiface@v1
      with:
        name: python-lucene_${{ github.sha }}_amd64
        path: deb
```

文中のDockerfileはリポジトリに含まれているものを使用している｡

# 参考文献

READMEの参考文献
https://knowledge.sakura.ad.jp/23478/

workflow-example-for-rust.ymlの参考文献
https://blog.takuchalle.dev/post/2020/10/22/github_actions_for_rust/