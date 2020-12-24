class Cup {
  constructor(label) {
    this.label = label;
    this.next = null;
    this.prev = null;
  }
}

class Circle {
  constructor(startingCups) {
    const cups = startingCups
      .map((c) => c instanceof Cup ? c : new Cup(c))
      .map((cup, i, arr) => {
        const next = i === arr.length - 1 ? arr[0] : arr[i + 1];
        const prev = i === 0 ? arr[arr.length - 1] : arr[i - 1];
        cup.next = next;
        cup.prev = prev;
        return cup;
      });
    this.current = cups[0];
  }

  next() {
    this.current = this.current.next;
  }

  prev() {
    this.current = this.current.prev;
  }

  min() {
    let seen = '';
    let min = 0;
    while (!seen.includes(this.current.label.toString())) {
      min = Math.min(min, this.current.label);
      seen += this.current.label;
      this.next();
    }
    return min;
  }
  
  max() {
    let seen = '';
    let max = 0;
    while (!seen.includes(this.current.label.toString())) {
      max = Math.max(max, this.current.label);
      seen += this.current.label;
      this.next();
    }
    return max;
  }

  print() {
    let seen = '';
    while (!seen.includes(this.current.label.toString())) {
      seen += this.current.label;
      this.next();
    }
    return seen;
  }

  includes(label) {
    let seen = '';
    while (!seen.includes(this.current.label.toString())) {
      seen += this.current.label;
      this.next();
    }
    return seen.includes(label);
  }

  slideTo(label) {
    while (this.current.label !== label) {
      this.next();
    }
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
    let seen = '';
    const _curr = this.current;
    const _next = this.current.next;

    while (true) {
      if (seen.includes(circle.current.label.toString())) {
        break;
      }
      
      this.current.next = new Cup(circle.current.label);
      this.current.next.prev = _curr;
      this.current.next.next = _next;

      seen += this.current.next.label;
      this.next();
      circle.next();
    }

    this.slideTo(_curr.label);
  }
}

function move(circle, max, min) {
  console.log(`cups: ${circle.print().split('').join(',')}`)
  const _curr = circle.current;
  const slice = circle.extract(3);
  console.log(`pick up: ${slice.print().split('').join(',')}`)
  let destination = circle.current.label - 1;

  while (true) {
    if (destination >= min && circle.includes(destination)) {
      break;
    }

    if (destination < min) {
      destination = max;
      continue;
    }

    destination -= 1;
  }

  console.log(`destination: ${destination}`)
  console.log('\n')

  circle.slideTo(destination).insert(slice);
  circle.slideTo(_curr.label);
  circle.next();
}

function runMoves(cups, num = 100) {
  const circle = new Circle(cups);
  const max = circle.max();
  const min = circle.min();


  for (let count = 0; count < num; count++) {
    console.log(`-- move ${count + 1} --`);
    move(circle, max, min);
  }

  return circle;
}

function partOne() {
  const input = "643719258".split("").map((c) => parseInt(c));
  return runMoves(input, 100).slideTo(1).print().slice(1);
}

console.log("Part 1: ", partOne());
