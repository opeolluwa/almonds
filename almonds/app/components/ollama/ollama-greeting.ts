export function getGreeting(username?: string): string {
  const greetings = {
    0: ["Happy Sunday", "Enjoy your Sunday"],
    1: ["Good Monday morning", "Monday — let’s get started"],
    2: ["Good Tuesday", "Tuesday in full swing"],
    3: ["Happy Wednesday", "Halfway there — keep going"],
    4: ["Good Thursday", "Thursday — almost weekend"],
    5: ["Happy Friday", "Friday — well done"],
    6: ["Happy Saturday", "Enjoy your Saturday"],

    morning: ["Good morning", "Rise & shine"],
    afternoon: ["Good afternoon", "Afternoon check-in"],
    evening: ["Good evening", "Evening vibes"],
    late: ["Good evening", "Late night thoughts"],

    newMonth: ["Happy New Month", "New month, new goals"],
    newYear: ["Happy New Year", "New Year, new beginnings"],
    christmas: ["Merry Christmas", "Season’s greetings"],
    valentine: ["Happy Valentine’s Day", "Love is in the air"],
  };

  const getRandom = (arr: string[]) =>
    arr[Math.floor(Math.random() * arr.length)];

  const now = new Date();
  const day = now.getDay();
  const h = now.getHours();
  const date = now.getDate();
  const month = now.getMonth();

  let parts: string[] = [];

  // Special days (highest priority)
  if (month === 0 && date === 1) {
    parts.push(getRandom(greetings.newYear));
  } else if (month === 11 && date === 25) {
    parts.push(getRandom(greetings.christmas));
  } else if (month === 1 && date === 14) {
    parts.push(getRandom(greetings.valentine));
  } else if (date === 1) {
    parts.push(getRandom(greetings.newMonth));
  }

  // Time of day
  if (h >= 5 && h < 11) {
    parts.push(getRandom(greetings.morning));
  } else if (h >= 11 && h < 17) {
    parts.push(getRandom(greetings.afternoon));
  } else if (h >= 17 && h <= 23) {
    parts.push(getRandom(greetings.evening));
  } else {
    parts.push(getRandom(greetings.late));
  }

  // Add weekday if only time was added
  if (parts.length === 1) {
    parts.unshift(getRandom(greetings[day]));
  }

  let message = parts.join(" • ");

  if (username) {
    message += `, ${username}`;
  }

  return message;
}
