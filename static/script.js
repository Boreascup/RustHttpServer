// 定义店铺和食物数据
const stores = {
  榕园: ["榕园一楼", "榕园二楼"],
  荔园: ["荔园一楼", "荔园二楼"],
  槿园: ["槿园一楼", "槿园二楼", "槿园三楼"],
  若海: ["若海一楼", "若海二楼"],
};

const foods = {
  榕园一楼: ["自选", "猪脚饭", "鱼粉", "水饺"],
  榕园二楼: ["农家菜", "自选", "潮汕粿条", "铁板", "小面", "牛肉饭", "烧腊"],
  荔园一楼: ["自选", "面"],
  荔园二楼: [
    "滑蛋饭",
    "烧腊",
    "麻辣烫",
    "螺蛳粉",
    "烤肉饭",
    "牛肉饭",
    "水饺",
    "黄焖鸡",
    "小炒",
  ],
  槿园一楼: ["包点", "面", "云吞", "粿条"],
  槿园二楼: ["自选", "猪脚饭", "汉堡"],
  槿园三楼: [
    "小炒",
    "面",
    "焗饭",
    "烤肉饭",
    "牛腩饭",
    "蒸饭",
    "煲仔饭",
    "冬阴功",
    "香锅",
    "麻辣烫",
    "炸鸡",
  ],
  若海一楼: ["肠粉", "车仔面", "肉夹馍", "汉堡", "螺蛳粉", "鸡丝凉面", "饺子"],
  若海二楼: ["烧腊", "牛肉饭", "自选", "小炒", "麻辣烫"],
};

let selectedStore = ""; // 声明全局变量用于存储当前选择的店铺
let selectedFloor = ""; // 声明全局变量用于存储当前选择的楼层
let retryCount = 0; // 用于计数重试次数

// 用户选择是否有心仪的店铺
function chooseStore(hasStore) {
  if (hasStore) {
    document.getElementById("choice").style.display = "none";
    document.getElementById("storeChoice").style.display = "block";
  } else {
    document.getElementById("choice").style.display = "none";
    const storeKeys = Object.keys(stores);
    selectedStore = storeKeys[Math.floor(Math.random() * storeKeys.length)]; // 随机选择店铺
    const storeFloors = stores[selectedStore];
    selectedFloor = storeFloors[Math.floor(Math.random() * storeFloors.length)]; // 随机选择楼层
    document.getElementById("retryChoice").style.display = "block";
    document.getElementById(
      "selectedStore"
    ).innerText = `${selectedStore} (${selectedFloor})`;
  }
}

// 用户选择店铺和楼层后选择食物
function selectFood1() {
  selectedStore = document.getElementById("store").value; // 更新选定的店铺
  selectedFloor = document.getElementById("floor").value; // 更新选定的楼层
  const selectedFood =
    foods[selectedFloor][
      Math.floor(Math.random() * foods[selectedFloor].length)
    ];
  displayResult(selectedFood);
}

// 用户选择店铺后选择食物（没有心仪店铺的情况下）
function selectFood2() {
  let store = document.getElementById("store").value;
  let floor = document.getElementById("floor").value;
  if (document.getElementById("retryChoice").style.display === "block") {
    store = selectedStore; // 使用之前选定的店铺
    floor = selectedFloor; // 使用之前选定的楼层
  }
  const selectedFood =
    foods[floor][Math.floor(Math.random() * foods[floor].length)];
  displayResult(selectedFood);
}

// 重选店铺
function retryStore() {
  retryCount++;
  if (retryCount >= 10) {
    alert("你都抽10次了，看得出饭堂很无聊，出去吃吧好吗？好的。");
    retryCount = 0;
    return;
  }

  document.getElementById("result").style.display = "none";
  document.getElementById("retryChoice").style.display = "none";
  const storeKeys = Object.keys(stores);
  selectedStore = storeKeys[Math.floor(Math.random() * storeKeys.length)]; // 更新选定的店铺
  const storeFloors = stores[selectedStore];
  selectedFloor = storeFloors[Math.floor(Math.random() * storeFloors.length)]; // 更新选定的楼层
  document.getElementById(
    "selectedStore"
  ).innerText = `${selectedStore} (${selectedFloor})`; // 更新显示在界面上的选定店铺和楼层的文字
  document.getElementById("retryChoice").style.display = "block"; // 重新显示重试选择
}

// 用户选择是否要重选食物
function retryFood() {
  retryCount++;
  if (retryCount >= 10) {
    alert("你都抽10次了，看得出饭堂很不对你胃口，出去吃吧好吗？好的。");
    retryCount = 0;
    return;
  }

  const selectedFood =
    foods[selectedFloor][
      Math.floor(Math.random() * foods[selectedFloor].length)
    ];
  displayResult(selectedFood);
}

// 显示选择结果
function displayResult(food) {
  document.getElementById("storeChoice").style.display = "none";
  document.getElementById("retryChoice").style.display = "none";
  document.getElementById("result").style.display = "block";
  document.getElementById("food").innerText = food;
}

// 根据选择的店铺动态更新楼层选项
function updateFloors() {
  const store = document.getElementById("store").value;
  const floorSelect = document.getElementById("floor");
  floorSelect.innerHTML = "";

  stores[store].forEach((floor) => {
    const option = document.createElement("option");
    option.value = floor;
    option.text = floor;
    floorSelect.appendChild(option);
  });
}
