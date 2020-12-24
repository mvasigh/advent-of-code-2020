class Cup {
  constructor(label) {
    this.label = label;
    this.next = null;
    this.prev = null;
  }
}

class Circle {
  constructor(startingCups, _size = null) {
    const max = Math.max(...startingCups);
    const size = _size ?? startingCups.length;

    const cups = Array(size)
      .fill(null)
      .map((c, i) => {
        if (startingCups[i]) {
          return startingCups[i] instanceof Cup
            ? new Cup(startingCups[i].label)
            : new Cup(startingCups[i]);
        } else {
          let delta = i + 1 - startingCups.length;
          return new Cup(max + delta);
        }
      })
      .map((cup, i, arr) => {
        const next = i === arr.length - 1 ? arr[0] : arr[i + 1];
        const prev = i === 0 ? arr[arr.length - 1] : arr[i - 1];
        cup.next = next;
        cup.prev = prev;
        return cup;
      });

    this.cups = cups.reduce((acc, curr) => {
      acc[curr.label] = curr;
      return acc;
    }, {});

    this.current = cups[0];
  }

  next() {
    this.current = this.current.next;
    return this;
  }

  prev() {
    this.current = this.current.prev;
    return this;
  }

  forEach(fn) {
    let start = this.current.label;
    do {
      fn(this.current);
      this.next();
    } while (this.current.label !== start);
  }

  min() {
    let min = 100;
    this.forEach((cup) => (min = Math.min(min, cup.label)));
    return min;
  }

  max() {
    let max = 0;
    this.forEach((cup) => (max = Math.max(max, cup.label)));
    return max;
  }

  print() {
    let printed = "";
    this.forEach((cup) => (printed += cup.label + ","));
    return printed.replace(/,$/, "");
  }

  includes(label) {
    return Boolean(this.cups[label]);
  }

  slideTo(label) {
    this.current = this.cups[label];
    return this;
  }

  extract(num = 1) {
    const slice = [];
    for (let i = 0; i < num; i++) {
      const cup = slice.length ? slice[slice.length - 1] : this.current;
      slice.push(cup.next);
    }
    this.current.next = slice[slice.length - 1].next;
    return new Circle(slice);
  }

  insert(circle) {
    let first = circle.current;

    const _curr = this.current;
    const _next = this.current.next;

    do {
      const own = this.cups[circle.current.label];

      this.current.next = own;
      own.next = _next;
      own.prev = _curr;

      this.current.next.prev = _curr;
      this.current.next.next = _next;

      this.next();
      circle.next();
    } while (circle.current.label !== first.label);

    this.slideTo(_curr.label);
  }
}

function move(circle, max, min) {
  const _curr = circle.current;
  const slice = circle.extract(3);
  let destination = circle.current.label - 1;

  while (true) {
    if (destination >= min && !slice.includes(destination)) {
      break;
    }

    if (destination < min) {
      destination = max;
      continue;
    }

    destination -= 1;
  }

  circle.slideTo(destination).insert(slice);
  circle.slideTo(_curr.label);
  circle.next();
}

function runMoves(cups, num = 100, size = null) {
  const circle = new Circle(cups, size);
  const max = circle.max();
  const min = circle.min();

  for (let count = 0; count < num; count++) {
    move(circle, max, min);
  }

  return circle;
}

function partOne() {
  const input = "643719258".split("").map((c) => parseInt(c));
  return runMoves(input, 100).slideTo(1).print().substring(2).replace(/,/g, "");
}

function partTwo() {
  const input = "643719258".split("").map((c) => parseInt(c));
  const circle = runMoves(input, 10_000_000, 1_000_000).slideTo(1);
  const cup1 = circle.next().current;
  const cup2 = circle.next().current;
  return cup1.label * cup2.label;
}

console.log("Part 1: ", partOne());
console.log("Part 2: ", partTwo());
