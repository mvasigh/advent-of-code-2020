const data = require('./data');

class Tile {
  constructor({ id, rows, rotation = 0, flipX = false, flipY = false }) {
    this.id = id;
    this.rows = rows.map((row) => row.split('').map((c) => (c === '#' ? 1 : 0)));
    this.rotation = rotation;
    this._flipX = flipX;
    this._flipY = flipY;
  }

  toJSON() {
    return {
      id: this.id,
      rows: this.rows.map((r) => r.map((c) => (c ? '#' : '.')).join('')),
      edges: this.edges(),
    };
  }

  clone() {
    return new Tile({
      id: this.id,
      rows: this.rows.map((row) => row.map((c) => (c ? '#' : '.')).join('')),
      rotation: this.rotation,
      flipX: this._flipX,
      flipY: this._flipY,
    });
  }

  stringify() {
    return this.rows.map((r) => r.map((el) => (el ? '#' : '.')).join('')).join('\n');
  }

  size() {
    return this.rows.length;
  }

  edges() {
    return this.rows.reduce(
      (acc, curr, i) => {
        if (i === 0) {
          acc.n = curr.join('');
        } else if (i === curr.length - 1) {
          acc.s = curr.join('');
        }
        acc.w += curr[0];
        acc.e += curr[curr.length - 1];
        return acc;
      },
      { n: '', s: '', e: '', w: '' }
    );
  }

  trim() {
    this.rows = this.rows
      .map((row) => {
        row.shift();
        row.pop();
        return row;
      })
      .filter((_, i, arr) => i !== 0 && i !== arr.length - 1);
  }

  rotate() {
    const len = this.rows.length;
    const rotated = Array(len)
      .fill(null)
      .map((r) => Array(len).fill(null));
    for (let i = 0; i < len; i++) {
      for (let j = 0; j < len; j++) {
        rotated[i][j] = this.rows[len - j - 1][i];
      }
    }
    this.rows = rotated;
    this.rotation += 90;
    return this;
  }

  flipX() {
    this.rows.forEach((row) => row.reverse());
    this._flipX = !this._flipX;
    return this;
  }

  flipY() {
    this.rows.reverse();
    this._flipY = !this._flipY;
    return this;
  }
}

function createTiles() {
  return data.map((rawTile) => new Tile(rawTile));
}

function getPermutations(tile) {
  const permutations = [tile];
  permutations.push(tile.clone().flipX());
  permutations.push(tile.clone().flipY());
  const rotated = permutations
    .map((perm) => {
      const rotations = [perm];
      rotations.push(perm.clone().rotate());
      rotations.push(perm.clone().rotate().rotate());
      rotations.push(perm.clone().rotate().rotate().rotate());
      return rotations;
    })
    .reduce((acc, curr) => acc.concat(curr), []);
  return rotated;
}

function getAllEdges(tiles) {
  return tiles.reduce((acc, tile) => {
    const permutations = getPermutations(tile);
    for (let p of permutations) {
      const edges = p.edges();
      for (let [dir, edge] of Object.entries(edges)) {
        if (!acc[edge]) {
          acc[edge] = {};
        }
        if (!acc[edge][dir]) {
          acc[edge][dir] = [];
        }
        acc[edge][dir].push(p);
      }
    }
    return acc;
  }, {});
}

function getPairedEdges(allEdges) {
  const uniques = {};
  const matches = {};
  const pairs = [
    ['n', 's'],
    ['e', 'w'],
  ];
  outer: for (let [edge, dirs] of Object.entries(allEdges)) {
    let unique = true;
    for (let [_1, _2] of pairs) {
      const candidates = dirs[_1];
      for (let c of candidates) {
        const eligible = dirs[_2].find((tile) => tile.id !== c.id);
        if (eligible) {
          if (!matches[edge]) {
            matches[edge] = {};
          }
          if (!matches[edge][_1]) {
            matches[edge][_1] = [];
          }
          if (!matches[edge][_2]) {
            matches[edge][_2] = [];
          }
          matches[edge][_1].push(c.id);
          matches[edge][_2].push(eligible.id);
          unique = false;
        }
      }
    }
    if (unique) {
      uniques[edge] = dirs;
    }
  }
  return { uniques, matches };
}

function getTilesFromEdges(edges) {
  const tileSet = Object.values(edges)
    .map((dirs) => Object.values(dirs))
    .flat()
    .flat()
    .reduce((acc, curr) => {
      if (!acc[curr.id]) {
        acc[curr.id] = curr;
      }
      return acc;
    }, {});
  return Object.values(tileSet);
}

function getCornerTiles(tiles, uniqueEdges) {
  const corners = [];
  const pairs = [
    ['n', 'e'],
    ['n', 'w'],
    ['s', 'e'],
    ['s', 'w'],
  ];

  for (let tile of tiles) {
    let edges = tile.edges();
    for (let [_1, _2] of pairs) {
      if (uniqueEdges[edges[_1]] && uniqueEdges[edges[_2]]) {
        corners.push({ tile, dir: _1 + _2 });
      }
    }
  }
  return corners;
}

function getNeighbors(grid, x, y) {
  return [
    ['n', grid[y - 1]?.[x]],
    ['e', grid[y]?.[x + 1]],
    ['s', grid[y + 1]?.[x]],
    ['w', grid[y]?.[x - 1]],
  ].reduce((acc, [key, val]) => {
    if (val) {
      acc[key] = val;
    }
    return acc;
  }, {});
}

function createTileFromImage(image) {
  // TODO: Rewrite
  // First, trim the tile
  image.forEach((col) => col.forEach((tile) => tile.trim()));

  // Next, figure out how big the image is
  const tileSize = image[0][0].size();
  const size = tileSize * image.length;

  // Now, make a 2d array for the image size
  const stitched = Array(size)
    .fill(null)
    .map(() => Array(size).fill(null));

  // Fill each pixel in with the corresponding value from a nested tile
  for (let i = 0; i < size; i++) {
    for (let j = 0; j < size; j++) {
      const tile = image[Math.floor(i / tileSize)][Math.floor(j / tileSize)];
      const value = tile.rows[i % tileSize][j % tileSize];
      stitched[i][j] = value;
    }
  }

  return new Tile({
    rows: stitched.map((col) => col.map((c) => (c ? '#' : '.')).join('')),
  })
    .rotate()
    .rotate();
}

function createImageFromCorner(size = 10, corner, edges) {
  const turns = { sw: 1, nw: 0, ne: 3, se: 2 };
  const opposites = { n: 's', e: 'w', s: 'n', w: 'e' };

  const { tile, dir: cornerDir } = corner;
  const usedTiles = new Set([corner.tile.id]);
  const image = Array(size)
    .fill(null)
    .map(() => Array(size).fill(null));

  let x = 0;
  let y = 0;

  for (let i = 0; i < turns[cornerDir]; i++) {
    tile.rotate();
  }

  image[y][x] = tile;

  while (usedTiles.size < size * size) {
    let currX = x % size;
    let currY = y % size;

    const neighbors = getNeighbors(image, currX, currY);
    const currTile = image[y][x];

    if (!currTile) {
      const candidates = [];
      for (let [dir, tile] of Object.entries(neighbors)) {
        candidates.push(...edges[tile.edges()[opposites[dir]]][dir].filter((tile) => !usedTiles.has(tile.id)));
      }
      image[currY][currX] = candidates[0];
      usedTiles.add(candidates[0].id);
    }

    x += 1;
    if (x > 0 && x % size === 0) {
      y += 1;
    }
  }
  return image;
}

function findSeaMonsters(_tile) {
  let monsters = 0;

  const monster = ['                  # ', '#    ##    ##    ###', ' #  #  #  #  #  #   '].map((r) =>
    r.split('').map((char) => char === '#')
  );
  const permutations = getPermutations(_tile);

  let monsterPixels;

  for (let tile of permutations) {
    if (monsters > 0) {
      break;
    }

    let x1 = 0;
    let x2 = monster[0].length - 1;
    let y1 = 0;
    let y2 = monster.length - 1;

    monsterPixels = new Set();

    while (x2 < tile.rows[0].length && y2 < tile.rows.length) {
      let isMonster = true;
      const hashes = [];

      outer: for (let i = 0; i < monster.length; i++) {
        for (let j = 0; j < monster[0].length; j++) {
          const checkMonster = monster[i][j];
          if (checkMonster && !tile.rows[y1 + i][x1 + j]) {
            isMonster = false;
            break outer;
          } else if (checkMonster && tile.rows[y1 + i][x1 + j]) {
            hashes.push(`${y1 + i},${x1 + j}`);
          }
        }
      }

      if (isMonster) {
        for (let hash of hashes) {
          monsterPixels.add(hash);
        }
        monsters += 1;
      }

      if (x2 === tile.rows[0].length - 1) {
        x1 = 0;
        x2 = monster[0].length - 1;
        y1 += 1;
        y2 += 1;
      } else {
        x1 += 1;
        x2 += 1;
      }
    }
  }

  return (
    _tile.rows.map((col) => col.reduce((acc, curr) => acc + curr)).reduce((acc, curr) => acc + curr) -
    monsterPixels.size
  );
}

function partOne() {
  const { uniques: uniqueEdges } = getPairedEdges(getAllEdges(createTiles()));
  const corners = getCornerTiles(getTilesFromEdges(uniqueEdges), uniqueEdges);
  return corners.reduce((acc, curr) => (acc *= curr.tile.id), 1);
}

function partTwo() {
  const tiles = createTiles();
  const allEdges = getAllEdges(tiles);
  const { uniques: uniqueEdges } = getPairedEdges(allEdges);
  const [corner] = getCornerTiles(getTilesFromEdges(uniqueEdges), uniqueEdges);
  const image = createImageFromCorner(Math.sqrt(tiles.length), corner, allEdges);
  const tile = createTileFromImage(image);
  return findSeaMonsters(tile);
}

console.log('Part 1: ', partOne());
console.log('Part 2: ', partTwo());
