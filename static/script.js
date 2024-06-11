// 定义店铺和饮品数据
const stores = ["瑞幸", "喜茶", "百分茶", "一点点"];
const drinks = {
  瑞幸: [
    "酱香拿铁",
    "夏日青提拿铁",
    "陨石拿铁",
    "碧螺知春拿铁",
    "冰厚乳拿铁",
    "冰吸生椰拿铁",
    "加浓美式",
    "标准美式",
    "冰镇杨梅瑞纳冰",
    "抹茶瑞纳冰",
    "芒果茉莉瑞纳冰",
    "巧克力瑞纳冰",
    "浮云朵朵瑞纳冰",
    "卡布奇诺瑞纳冰",
    "茉莉奶绿瑞纳冰",
    "云南小甘橘·澳瑞白",
    "云南小甘橘·拿铁",
    "云南小甘橘·美式",
    "云南小甘橘·Dirty",
    "杏花乌龙拿铁",
    "茉莉海盐拿铁",
    "茉莉花香拿铁",
    "陨石生椰拿铁",
    "生椰丝绒拿铁",
    "厚乳拿铁",
    "抹茶纳瑞冰",
    "一杯巧克力",
    "纯牛奶",
    "橙C美式",
    "柚C美式",
    "柠C美式",
    "橙C冰茶",
    "柚C冰茶",
    "杨梅冰茶",
    "耶加•Dirty",
    "耶加雪菲•美式",
    "云南小柑橘•Dirty",
    "昆仑煮雪拿铁",
    "冲绳黑糖拿铁",
    "桂花龙井拿铁",
    "茉莉海盐拿铁",
    "生椰拿铁",
    "椰皇拿铁",
    "生酪拿铁",
    "马斯卡彭生酪拿铁",
    "褚橙拿铁",
    "生椰丝绒拿铁",
    "陨石生椰拿铁",
    "摸鱼生椰拿铁",
    "抓马西瓜拿铁",
    "生椰爱摩卡",
    "椰青冰萃美式",
  ],

  喜茶: [
    "芝芝多肉葡萄",
    "芝芝多肉青提",
    "芝芝芒芒",
    "芝芝多肉杨梅",
    "轻芝莓莓",
    "小奶茉（白茉香）",
    "酷黑莓桑",
    "绯红厚乳茶（嫩蜜香）",
    "女明星•蓝莓果 ",
    "轻芝莓莓",
    "多肉葡萄",
    "超多肉芒芒甘露",
    "超清爽百香果",
    "轻芝多肉葡萄",
    "椰椰芒芒",
    "超清爽百香芒果",
    "多肉青提",
    "多肉芒芒",
    "多肉桃李",
    "多肉杨梅",
    "绿妍打柠茶",
    "青提打柠茶",
    "多肉桑桑",
    "水云间（山韵香）",
    "芭比粉•水仙（岩韵香）",
    "芋泥牛乳茶",
    "烤黑糖波波牛乳茶",
    "仙草大满贯",
    "芋圆牛乳茶",
    "黑糖波波牛乳茶",
    "芋泥牛乳满贯",
    "鸭喜香轻柠茶",
    "绿妍轻柠茶",
    "青提轻柠茶",
    "轻波波牛乳茶",
    "轻波波牛乳",
    "青稞牛乳茶",
    "老丛厚乳茶（沉木香）",
    "轻芒芒甘露",
    "喜柿多多",
    "水牛乳双拼抹茶",
  ],

  百分茶: [
    "竹生空野",
    "茉立绣窗",
    "桂落满陇",
    "伯虎桃山",
    "元气娇颜膏方饮",
    "芋泥啵啵鲜奶",
    "芋泥啵啵鲜奶茶",
    "黑糖BiuBiu鲜奶",
    "芋圆啵啵文山青鲜奶茶",
    "栗子奶奶(无奶油顶)",
    "手剥红石榴奶缇",
    "手剥红石榴",
    "暴打红石榴柠檬茶",
    "栗子奶奶",
    "大橘青提",
    "大橘白巧",
    "栗子桂花拿铁",
    "鲜橙茉莉美式（整颗鲜橙）",
    "鲜葡萄茉莉美式",
    "楂心美式",
    "杨枝甘露（清爽版）",
    "芒椰小三圆",
    "手舂肉肉葡萄奶缇",
    "茉立青提",
    "手春肉肉葡萄（无奶缇）",
    "暴打葡萄柠檬茶",
    "暴打鸭屎香柠檬茶",
    "来一桶春茶十二月",
    "春茶十二月",
    "VC橙王",
    "燕麦奶绿豆冰",
    "燕麦奶绿豆冰宝宝",
    "黑糖BiuBiu宝宝",
  ],
  一点点: [
    "茉莉绿茶",
    "锦鲤红茶",
    "四季春茶",
    "清香乌龙茶",
    "抹茶",
    "红茶玛奇朵",
    "乌龙玛奇朵",
    "多多绿",
    "冰淇淋红茶",
    "1号四季春珍波椰",
    "蜂蜜锦鲤红",
    "珍珠奶茶",
    "珍珠奶绿",
    "珍珠红/乌",
    "珍珠绿/青",
    "奶茶",
    "椰果奶茶",
    "茉香奶綠",
    "仙草奶冻",
    "四季奶青",
    "乌龙奶茶",
    "奶绿装芒",
    "阿华田",
    "抹茶奶茶",
    "咖啡冻奶茶",
    "可可奶茶(黄金比例)",
    "咖啡冻奶茶",
    "牛乳红茶",
    "牛乳乌龙茶",
    "牛乳抹茶",
    "燕麦牛乳四季春茶",
    "翡翠柠檬",
    "百香三重奏",
    "柚心动了",
    "金桔柠檬",
    "柠檬多多",
    "葡萄柚绿",
    "茶冻芒果青",
    "柚子青",
    "茶冻柠檬芭乐绿",
    "海盐柠檬柚子乌",
    "杨枝甘露",
    "波霸奶茶",
    "波霸奶绿",
    "波霸红/烏",
    "波霸绿/青",
    "布丁奶茶",
    "波霸桂花奶乌",
    "海盐奶绿",
    "五福奶青",
    "紫米芋圆椰奶茶",
    "珍珠桂花乌龙",
    "焦糖海盐奶茶",
    "芋圆奶绿",
    "茶冻芭乐柠檬遇冰沙",
    "咖啡冻奶茶遇冰沙",
  ],
};

// 用户选择是否有心仪的店铺
let selectedStore = ""; // 声明全局变量用于存储当前选择的店铺
function chooseStore(hasStore) {
  if (hasStore) {
    document.getElementById("choice").style.display = "none";
    document.getElementById("storeChoice").style.display = "block";
  } else {
    document.getElementById("choice").style.display = "none";
    selectedStore = stores[Math.floor(Math.random() * stores.length)]; // 更新选定的店铺
    document.getElementById("retryChoice").style.display = "block";
    document.getElementById("selectedStore").innerText = selectedStore;
  }
}

// 用户选择店铺后选择饮品
// 用户有心仪店铺
function selectDrink1() {
  selectedStore = document.getElementById("store").value; // 更新选定的店铺
  const selectedDrink =
    drinks[selectedStore][
      Math.floor(Math.random() * drinks[selectedStore].length)
    ];
  displayResult(selectedDrink);
}

// 用户选择店铺后选择饮品
// 用户没有心仪店铺
function selectDrink2() {
  let selectedStore = document.getElementById("store").value;
  if (document.getElementById("retryChoice").style.display === "block") {
    selectedStore = document.getElementById("selectedStore").innerText; // 使用之前选定的店铺
  }
  const selectedDrink =
    drinks[selectedStore][
      Math.floor(Math.random() * drinks[selectedStore].length)
    ];
  displayResult(selectedDrink);
}

// 重选店铺
function retryStore() {
  document.getElementById("result").style.display = "none";
  document.getElementById("retryChoice").style.display = "none";
  selectedStore = stores[Math.floor(Math.random() * stores.length)]; // 更新选定的店铺
  document.getElementById("selectedStore").innerText = selectedStore; // 更新显示在界面上的选定店铺的文字
}

// 用户选择是否要重选饮品
function retryDrink() {
  const selectedDrink =
    drinks[selectedStore][
      Math.floor(Math.random() * drinks[selectedStore].length)
    ];
  displayResult(selectedDrink);
}

// 显示选择结果
function displayResult(drink) {
  document.getElementById("storeChoice").style.display = "none";
  document.getElementById("retryChoice").style.display = "none";
  document.getElementById("result").style.display = "block";
  document.getElementById("drink").innerText = drink;
}
