(() => {
    const SQUARE_ROOT = 4; // 4 x 4のタイルを想定。3x3にしたい場合はここを3にする
    const TILE_COUNT = Math.pow(SQUARE_ROOT, 2);
    const panelElement = document.getElementById("panel");

    // 配列を受け取りランダムソートして新しい配列を返却
    const shuffle = (array) => {
        for (let i = array.length - 1; i >= 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            [array[i], array[j]] = [array[j], array[i]];
        }
        return array;
    };

    // タイルの数分の配列を生成
    const tileContentArr = [...Array(TILE_COUNT).keys()];
    // ↑の配列をシャッフルし、0のみ空文字にする
    const shuffledTileContentArr = shuffle(tileContentArr).map((e) => e ? e.toString() : "");

    // タイルのクリックイベント
    const handleClick = (_event) => {
        const tileElements = Array.from(panelElement.children); // タイルの全要素を取得
        const target = _event.target;
        const currentTextContent = target.textContent; // クリックした要素の内容

        if (currentTextContent === "") return; // 空のタイルをクリックしたときは何もしない
        const elementIndex = Number(target.dataset.index); // data属性にindexを設定したのを取得 左上から数えた際の要素の順番

        const everyDirectionElements = { // クリックした要素の前後左右の要素を取得
            top: tileElements[elementIndex - SQUARE_ROOT], // 上
            bottom: tileElements[elementIndex + SQUARE_ROOT], // 下
            left: tileElements[elementIndex - 1], // 左
            right: tileElements[elementIndex + 1], // 右
        };
        const isEmptyAdjacent = Object.values(everyDirectionElements).some((el) => el && el.textContent === ""); // クリックした要素が空のタイルに隣接しているかどうか
        if (!isEmptyAdjacent) return; // 空のタイルに隣接していない場合は何もしない
        const emptyAdjacentElement = Object.values(everyDirectionElements).find((el) => el && el.textContent === ""); // 隣接している要素の中から空のタイルを取得
        emptyAdjacentElement.textContent = currentTextContent; // 空のタイルにクリックした要素の内容を代入
        target.textContent = "";  // クリックした要素を空にする
    };

    // タイル要素を必要分作成しHTMLに追加
    for (let i = 0; i < TILE_COUNT; i++) {
        const tileElement = document.createElement("div");
        tileElement.classList.add("tile");
        tileElement.textContent = shuffledTileContentArr[i];
        tileElement.dataset.index = i.toString(); // data属性にindexを設定 左上から数えたときの順番。上下左右の要素を取得するときに使用
        tileElement.addEventListener("click", handleClick); // clickイベントを設定
        panelElement.appendChild(tileElement);
    }
})();
