<!DOCTYPE html>
<html lang="uk">
<head>
  <meta charset="UTF-8">
  <title>Площа об'єднання прямокутників</title>
  <style>
    pre {
      font-family: monospace;
      font-size: 14px;
      background: #f4f4f4;
      padding: 10px;
      margin: 10px;
    }
    button {
      margin: 10px;
      padding: 8px 16px;
      font-size: 16px;
      cursor: pointer;
    }
  </style>
</head>
<body>
<pre id="output"></pre>
<button onclick="runDemo()">Запустити демонстрацію</button>

<script>
  class Point {
    constructor(x, y) {
      this.x = x;
      this.y = y;
    }
  }
  class Rectangle {
    constructor(a, b) {
      this.a = a;
      this.b = b;
    }
  }
  function area_occupied(rectangles) {
    if (!rectangles.length) return 0;
    let xs = new Set();
    let ys = new Set();
    for (let rect of rectangles) {
      xs.add(rect.a.x);
      xs.add(rect.b.x);
      ys.add(rect.a.y);
      ys.add(rect.b.y);
    }
    let xList = Array.from(xs).sort((a,b) => a - b);
    let yList = Array.from(ys).sort((a,b) => a - b);
    let totalArea = 0;
    for (let i = 0; i < xList.length - 1; i++) {
      for (let j = 0; j < yList.length - 1; j++) {
        let x1 = xList[i];
        let x2 = xList[i+1];
        let y1 = yList[j];
        let y2 = yList[j+1];
        let occupied = false;
        for (let rect of rectangles) {
          if (x1 >= rect.a.x && x2 <= rect.b.x && y1 <= rect.a.y && y2 >= rect.b.y) {
            occupied = true;
            break;
          }
        }
        if (occupied) {
          let width = x2 - x1;
          let height = y1 - y2; // бо y1 > y2
          totalArea += width * height;
        }
      }
    }
    return totalArea;
  }
  function test_data() {
    return [
      new Rectangle(new Point(2, 9), new Point(5, 3)),
      new Rectangle(new Point(1, 8), new Point(11, 6)),
      new Rectangle(new Point(9, 10), new Point(13, 2))
    ];
  }
  function runDemo() {
    let data = test_data();
    let occupied = area_occupied(data);
    let output = "=== ТЕСТОВИЙ ПРИКЛАД ===\n";
    output += "Прямокутники:\n";
    for (let i = 0; i < data.length; i++) {
      let rect = data[i];
      output += `  ${i+1}: (${rect.a.x},${rect.a.y}) - (${rect.b.x},${rect.b.y})\n`;
    }
    output += `\nОбчислена зайнята площа: ${occupied}\n`;
    output += `Очікувана площа: 60\n`;
    output += occupied === 60 ? "✅ ТЕСТ ПРОЙДЕНО" : "❌ ТЕСТ НЕ ПРОЙДЕНО";
    let rect1 = new Rectangle(new Point(0, 5), new Point(5, 0));
    let rect2 = new Rectangle(new Point(3, 4), new Point(8, 1));
    let area2 = area_occupied([rect1, rect2]);
    output += "\n\n=== ДОДАТКОВИЙ ТЕСТ (перетин) ===\n";
    output += `Прямокутник1: (0,5)-(5,0), площа = 25\n`;
    output += `Прямокутник2: (3,4)-(8,1), площа = 15\n`;
    output += `Сума площ = 40, але з перетином (3x3=9) => очікувана площа = 31\n`;
    output += `Обчислено: ${area2} → ${area2 === 31 ? "✅" : "❌"}`;

    document.getElementById("output").textContent = output;
    console.log(output);
  }
  runDemo();
</script>
</body>
</html>
