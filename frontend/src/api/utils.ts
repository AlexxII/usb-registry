export function analizeData(usbData: string) {
  const ITEMS_COUNT = 12;
  let lines = usbData.split("\n").filter(item => item !== "");

  lines.forEach((line, index) => {
    let device = line.split(";");
    if (device.length != ITEMS_COUNT) {
      throw Error(JSON.stringify({
        text: "Неправильное кол-во столбцов",
        line: index + 1
      }));
    }
  });
  return lines.join("\n");
}
