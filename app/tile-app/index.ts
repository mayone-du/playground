(() => {
  const SQUARE_ROOT = 4; // 4x4のタイルを想定
  const TILE_COUNT = Math.pow(SQUARE_ROOT, 2);

  const panelElement = document.getElementById("panel");

  // 配列を受け取りランダムソートして新しい配列を返却
  const shuffle = (array: number[]): number[] => {
    for (let i = array.length - 1; i >= 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [array[i], array[j]] = [array[j], array[i]];
    }
    return array;
  };

  // 長さ16の数値の配列生成
  const _tileContentArr = [...Array(TILE_COUNT).keys()]; // 次の1度のみ使用するため_を付ける
  // ↑の配列をシャッフルし、0のみ空文字にする
  const shuffledTileContentArr = shuffle(_tileContentArr).map((e) =>
    e ? e.toString() : ""
  );

  // タイルのクリックイベント
  const handleClick = (_event: MouseEvent) => {
    const tileElements = Array.from(panelElement.children); // タイルの要素を取得
    const target = _event.target as HTMLDivElement; // 型アサーションするため引数のeventは_eventにする
    const currentTextContent = target.textContent;
    // 空のタイルをクリックしたときは何もしない
    if (currentTextContent === "") return;
    const elementIndex = Number(target.dataset.index); // data属性にindexを設定したのを取得 左上から数えた際の要素の順番

    // クリックした要素の前後左右の要素を取得
    const everyDirectionElements = {
      top: tileElements[elementIndex - SQUARE_ROOT],
      bottom: tileElements[elementIndex + SQUARE_ROOT],
      left: tileElements[elementIndex - 1],
      right: tileElements[elementIndex + 1],
    };

    // からのタイルに隣接しているかどうかを判定
    const isEmptyAdjacent = Object.values(everyDirectionElements).some(
      (el) => el && el.textContent === ""
    );
    if (!isEmptyAdjacent) return; // からのタイルに隣接していない場合は何もしない
    const emptyAdjacentElement = Object.values(everyDirectionElements).find(
      (el) => el && el.textContent === ""
    );
    emptyAdjacentElement.textContent = currentTextContent;
    target.textContent = "";
  };

  // タイル要素を作成しHTMLに追加
  for (let i = 0; i < TILE_COUNT; i++) {
    const tileElement = document.createElement("div");
    tileElement.classList.add("tile");
    tileElement.textContent = shuffledTileContentArr[i];
    tileElement.dataset.index = i.toString(); // 左上から数えたときの要素の順番を取得するためdataset属性にindexを設定
    tileElement.addEventListener("click", handleClick); // Clickイベントを各タイル要素に追加
    panelElement.appendChild(tileElement);
  }
})();
