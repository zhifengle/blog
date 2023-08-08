function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function retry({ toTry, times = 5, interval = 1 }) {
  let attemptCount = 0;
  while (true) {
    try {
      const result = await toTry();
      return result;
    } catch (error) {
      if (++attemptCount >= times) throw error;
    }
    await sleep(interval);
  }
}
