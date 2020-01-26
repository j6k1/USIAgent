# USIAgent
## USIプロトコルに対応した将棋AIを開発するためのフレームワーク

USIAgentは、Rustを用いて将棋AIの標準通信プロトコルであるUSIプロトコルに対応した将棋AIを容易に開発できます。  

使い方は、USIPlayerトレイトを実装してUsiAgent構造体のインスタンスを生成してstartするだけです。

合法手の高速な列挙や盤面の状態への手の適用、盤面のハッシュ計算やそれを用いた千日手のチェックなど、一通りの機能はそろっています。

あなたもUSIAgentを使って将棋AIの開発を始めてみませんか？



### [ドキュメント](https://j6k1.github.io/USIAgent/usiagent/index.html)

