const data = require("./data");

class Tile {
  constructor({ id, rows, rotation = 0, flipX = false, flipY = false }) {
    this.id = id;
    this.rows = rows.map((row) =>
      row.split("").map((c) => (c === "#" ? 1 : 0))
    );
    this.rotation = rotation;
    this._flipX = flipX;
    this._flipY = flipY;
  }

  toJSON() {
    return {
      id: this.id,
      rows: this.rows.map((r) => r.map((c) => (c ? "#" : ".")).join("")),
      edges: this.edges(),
    };
  }

  clone() {
    return new Tile({
      id: this.id,
      rows: this.rows.map((row) => row.map((c) => (c ? "#" : ".")).join("")),
      rotation: this.rotation,
      flipX: this._flipX,
      flipY: this._flipY,
    });
  }

  edges() {
    return this.rows.reduce(
      (acc, curr, i) => {
        if (i === 0) {
          acc.n = curr.join("");
        } else if (i === curr.length - 1) {
          acc.s = curr.join("");
        }
        acc.w += curr[0];
        acc.e += curr[curr.length - 1];
        return acc;
      },
      { n: "", s: "", e: "", w: "" }
    );
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
    ["n", "s"],
    ["e", "w"],
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
    ["n", "e"],
    ["n", "w"],
    ["s", "e"],
    ["s", "w"],
  ];

  for (let tile of tiles) {
    let edges = tile.edges();
    for (let [_1, _2] of pairs) {
      if (uniqueEdges[edges[_1]] && uniqueEdges[edges[_2]]) {
        corners.push(tile);
      }
    }
  }
  return corners;
}

function partOne() {
  const tiles = createTiles();
  const allEdges = getAllEdges(tiles);
  const { uniques: uniqueEdges } = getPairedEdges(allEdges);
  const uniqueEdgeTiles = getTilesFromEdges(uniqueEdges);
  const corners = getCornerTiles(uniqueEdgeTiles, uniqueEdges);
  return corners.reduce((acc, curr) => (acc *= curr.id), 1);
}

function partTwo() {
  const tiles = createTiles();
  const allEdges = getAllEdges(tiles);
  const { matches: pairedEdges } = getPairedEdges(allEdges);
}

console.log("Part 1: ", partOne());
console.log("Part 2: ", partTwo());
