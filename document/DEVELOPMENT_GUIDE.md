## Sophisticate Phrase 開発手順書

このドキュメントは、Sophisticate Phrase アプリケーションの開発環境をセットアップし、運用するための手順を説明します。

### 1. 前提条件

以下のツールがホストマシンにインストールされていることを確認してください。

*   **Docker と Docker Compose**: コンテナ化されたデータベースを管理するため。
*   **Rust と Cargo**: Rustアプリケーションのビルドと実行のため。
*   **`sqlx-cli`**: SQLxのマイグレーションとクエリキャッシュの生成のため。
    *   インストール: `cargo install sqlx-cli --no-default-features --features postgres`
*   **`cargo-watch`**: 開発時の自動リロードのため。
    *   インストール: `cargo install cargo-watch`

### 2. 初期セットアップ (初回のみ、またはリポジトリをクローンした後)

1.  **プロジェクトファイルの権限修正 (必要に応じて)**:
    以前のDocker操作などにより、プロジェクトファイルの権限が正しくない場合があります。以下のコマンドで、現在のユーザーがプロジェクト内のすべてのファイルとディレクトリの所有者となり、書き込み権限が付与されます。
    ```bash
    sudo chown -R $(whoami):$(whoami) /home/gwrhyr/Project/sophisticate-phrase
    ```

### 3. 開発ワークフロー (ホストでRustアプリ、DockerでDB)

このワークフローは、開発時の高速なイテレーションを目的としています。Rustアプリケーションはホストマシンで実行され、DockerコンテナはPostgreSQLデータベースのみを提供します。

1.  **データベースコンテナのみを起動**:
    ```bash
    docker-compose up -d db
    ```
    *   これにより、PostgreSQLデータベースがバックグラウンドで起動します。

2.  **マイグレーションの実行 (データベースのリセット時、または新規マイグレーション追加時のみ)**:
    *   データベースを完全にリセットした場合（例: `docker-compose down -v` を実行した場合）や、新しいマイグレーションファイルを追加した場合は、以下のコマンドでマイグレーションを実行し、データベーススキーマを更新します。
    ```bash
    DATABASE_URL=postgres://user:password@localhost:5432/sophisticate_phrase_db sqlx migrate run
    ```

3.  **SQLx クエリキャッシュの生成/更新**:
    *   Rustコード内のSQLクエリを変更した場合、またはデータベーススキーマが変更された場合（マイグレーション実行後など）は、以下のコマンドでSQLxのクエリキャッシュを更新します。
    ```bash
    DATABASE_URL=postgres://user:password@localhost:5432/sophisticate_phrase_db cargo sqlx prepare
    ```
    *   このコマンドにより、`.sqlx/` ディレクトリ内のクエリ情報が更新されます。このディレクトリはバージョン管理下に置くことを推奨します。

4.  **Rustアプリケーションを `cargo watch` で起動 (ユーザー自身で実行)**:
    **注:** Geminiは現在 `cargo watch` を正常に実行できないため、以下のコマンドはユーザーが手動で実行する必要があります。
    ```bash
    DATABASE_URL=postgres://user:password@localhost:5432/sophisticate_phrase_db cargo watch -x run
    ```
    *   このコマンドは、Rustアプリケーションをコンパイルし、変更を監視して自動的に再コンパイル・再起動します。
    *   アプリケーションは `http://localhost:3000` でアクセス可能になります。

### 4. フルDocker化されたアプリケーション (テスト/本番環境向け)

このワークフローは、アプリケーション全体をDockerコンテナ内で実行し、本番環境に近い形でテストする場合や、実際にデプロイする場合に使用します。

1.  **すべてのコンテナを停止し、ボリュームを削除 (クリーンな状態から開始する場合)**:
    ```bash
    docker-compose down -v
    ```
    *   `-v` オプションは、データベースのデータボリュームを含むすべてのボリュームを削除し、データベースを完全にリセットします。

2.  **Dockerイメージをビルド**:
    ```bash
    docker-compose build --no-cache
    ```
    *   `--no-cache` オプションは、キャッシュを使用せずにイメージを再ビルドします。これにより、最新のコードと `.sqlx/` ディレクトリの内容がイメージに含まれることが保証されます。

3.  **データベースとアプリケーションコンテナを起動**:
    ```bash
    docker-compose up -d
    ```
    *   これにより、データベースコンテナとアプリケーションコンテナの両方がバックグラウンドで起動します。アプリケーションは起動時に自動的にマイグレーションを実行します。
    *   アプリケーションは `http://localhost:3000` でアクセス可能になります。
