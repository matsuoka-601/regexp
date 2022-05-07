# クレートに関するメモ
[ここ](https://doc.rust-jp.rs/book-ja/ch07-01-packages-and-crates.html)を参考に、クレートとモジュールについて整理してみる

## パッケージとクレート
- クレート: バイナリクレートか、ライブラリクレートのどちらか
- パッケージ: ある機能群を提供する一つ以上のクレート
    - パッケージのクレートを、どのようにビルドするかを説明するファイルが```Cargo.toml```
    - パッケージは、0個か1個のライブラリクレートを持っていないといけない（それ以上はダメ）
    - パッケージは、バイナリクレートはいくら持ってもよい
- ```src/main.rs```は、パッケージと同じ名前（ここだったら```regexp```）を持つバイナリクレートのクレートルート
- ```src/lib.rs```は、パッケージと同じ名前を持つライブラリクレートのクレートルート

## モジュール
- モジュール: クレート内のコードをグループ化する
    - コードの可読性と再利用性を上げる
    - 要素のプライバシーも制御できる (publicかprivateか)
- 以下のようなコードを```src/lib.rs```に書いたとする。

    ```rust
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}

            fn seat_at_table() {}
        }

        mod serving {
            fn take_order() {}

            fn serve_order() {}

            fn take_payment() {}
        }
    }
    ```

    このようにすると、以下のようなモジュールツリーが生成される。

    ```
    crate
    └── front_of_house
        ├── hosting
        │   ├── add_to_waitlist
        │   └── seat_at_table
        └── serving
            ├── take_order
            ├── serve_order
            └── take_payment
    ```

    ```crate```というモジュールは、```src/lib.rs```によって作られるものである。```src/lib.rs```（や```src/main.rs```）がクレートルートと呼ばれているのは、このようにクレートのモジュール構造の根っこに```crate```モジュールを作るためである。

- どうも