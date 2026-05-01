<!DOCTYPE html>
<html lang="uk">
<head>
  <meta charset="UTF-8">
  <title>Розподіл вантажу на кораблях</title>
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
  function count_permutation(shipments) {
    if (!Array.isArray(shipments) || shipments.length === 0) return 0;
    const total = shipments.reduce((sum, val) => sum + val, 0);
    const n = shipments.length;
    if (total % n !== 0) {
      return -1;
    }
    const average = total / n;
    let positiveDiffSum = 0;
    for (let val of shipments) {
      const diff = val - average;
      if (diff > 0) {
        positiveDiffSum += diff;
      }
    }
    return positiveDiffSum;
  }
  function gen_shipments(n) {
    if (n <= 0) return [];
    let shipments = [];
    let sum = 0;
    for (let i = 0; i < n; i++) {
      let val = Math.floor(Math.random() * 100) + 1;
      shipments.push(val);
      sum += val;
    }
    const remainder = sum % n;
    if (remainder !== 0) {
      shipments[n-1] += (n - remainder);
      if (shipments[n-1] < 0) shipments[n-1] = 0;
    }
    return shipments;
  }
  function analyzeShipments(shipments) {
    const n = shipments.length;
    const total = shipments.reduce((a,b)=>a+b,0);
    const average = total / n;
    const permutations = count_permutation(shipments);
    const possible = (total % n === 0);

    let output = `Кораблів: ${n}\n`;
    output += `Вантаж: [${shipments.join(", ")}]\n`;
    output += `Сума: ${total}, Середнє: ${average}\n`;
    if (possible) {
      output += `✅ Розподіл можливий.\n`;
      output += `Мінімальна кількість переносів вантажу: ${permutations}\n`;
      // Покажемо деталі
      const target = average;
      let diffs = shipments.map(v => v - target);
      output += `Різниці: [${diffs.map(d => d>=0? `+${d}` : `${d}`).join(", ")}]\n`;
      let posSum = diffs.filter(d => d>0).reduce((a,b)=>a+b,0);
      let negSum = -diffs.filter(d => d<0).reduce((a,b)=>a+b,0);
      output += `Сума додатних різниць: ${posSum}, сума від'ємних (за модулем): ${negSum}\n`;
    } else {
      output += `❌ Розподіл порівну НЕМОЖЛИВИЙ, оскільки сума ${total} не ділиться на ${n} без остачі.\n`;
      output += `Функція count_permutation повернула: ${permutations} (означає помилку)\n`;
    }
    return output;
  }
  function runDemo() {
    let allOutput = "";
    const shipments1 = [8, 2, 2, 4, 4];
    allOutput += "=== ПРИКЛАД 1 ===\n";
    allOutput += analyzeShipments(shipments1) + "\n\n";
    const shipments2 = [9, 3, 7, 2, 9];
    allOutput += "=== ПРИКЛАД 2 ===\n";
    allOutput += analyzeShipments(shipments2) + "\n\n";
    allOutput += "=== ВИПАДКОВИЙ ВЕКТОР (gen_shipments) ===\n";
    const randomVec = gen_shipments(6);
    allOutput += analyzeShipments(randomVec) + "\n\n";
    allOutput += "=== НЕМОЖЛИВИЙ ВИПАДОК (для демонстрації) ===\n";
    const impossible = [1, 2, 3, 4];
    allOutput += analyzeShipments(impossible) + "\n\n";
    allOutput += "=== ВІДПОВІДІ НА ПИТАННЯ (пункти 3-4) ===\n";
    allOutput += "3. Чи завжди можливо всі кораблі забезпечити однаковою кількістю грузу?\n";
    allOutput += "   Ні, тільки якщо загальна сума вантажу ділиться на кількість кораблів без остачі.\n";
    allOutput += "4. Як буде виглядати сигнатура в іншому випадку? (Rust)\n";
    allOutput += "   Можна використати Result<usize, &str> або Option<usize>. Наприклад:\n";
    allOutput += "   fn count_permutation(shipments: &Vec<u32>) -> Option<usize>\n";
    allOutput += "   або -> Result<usize, String>. У JavaScript ми повертаємо -1 при помилці.\n";

    document.getElementById("output").textContent = allOutput;
    console.log(allOutput);
  }
  runDemo();
</script>
</body>
</html>
