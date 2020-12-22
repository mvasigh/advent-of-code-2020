const data = require("./data");

const RECIPE_RE = /(^[a-z\s]+) \(contains ([a-z, ]+)\)?/;

class Recipe {
  constructor({ ingredients, allergens }) {
    this.ingredients = new Set(ingredients);
    this.allergens = new Set(allergens);
  }
}

const getAll = (property) => (recipes) =>
  recipes.reduce((acc, recipe) => {
    for (let item of recipe[property]) {
      acc.add(item);
    }
    return acc;
  }, new Set());

function getNonAllergenIngredients(recipes) {
  const ingredients = getAll("ingredients")(recipes);
  const safe = new Set();

  // For each ingredient...
  for (let ingredient of ingredients) {
    // 1. Find all recipes with it that have allergens and add them
    const eligible = recipes.reduce((acc, curr) => {
      if (curr.ingredients.has(ingredient)) {
        for (let allergen of curr.allergens) {
          acc.add(allergen);
        }
      }
      return acc;
    }, new Set());

    // 2. Find another recipe with each allergen that doesn't have the ingredient
    const disqualified = new Set();
    for (let recipe of recipes) {
      for (let allergen of eligible) {
        if (
          recipe.allergens.has(allergen) &&
          !recipe.ingredients.has(ingredient)
        ) {
          disqualified.add(allergen);
        }
      }
    }

    // 3. If all allergens are eliminated, it is safe
    if (eligible.size === disqualified.size) {
      safe.add(ingredient);
    }
  }

  return safe;
}

const createCounter = (property) => (recipes, phrase) => {
  let num = 0;
  for (let recipe of recipes) {
    if (recipe[property].has(phrase)) {
      num += 1;
    }
  }
  return num;
};

function createRecipes() {
  return data.map((recipe) => {
    let match = RECIPE_RE.exec(recipe);
    let [_, ingredients, allergens] = match;

    return new Recipe({
      ingredients: ingredients.trim().split(" "),
      allergens: allergens.trim().split(", "),
    });
  });
}

function getIngredientAllergens(recipes, inerts) {
  // First, preprocess recipes to remove any inert ingredients
  for (let recipe of recipes) {
    for (let ingredient of recipe.ingredients) {
      if (inerts.has(ingredient)) {
        recipe.ingredients.delete(ingredient);
      }
    }
  }

  // Next, for each allergen, narrow potential ingredients down
  const allergens = getAll("allergens")(recipes);
  let matches = [...allergens].reduce(
    (acc, curr) => ({ ...acc, [curr]: [] }),
    {}
  );

  for (let allergen of allergens) {
    const possible = recipes.reduce((acc, curr) => {
      if (curr.allergens.has(allergen)) {
        for (let i of curr.ingredients) {
          acc.add(i);
        }
      }
      return acc;
    }, new Set());
    let disqualified = new Set();

    for (let [i, recipe] of Object.entries(recipes)) {
      if (!recipe.allergens.has(allergen)) continue;

      for (let ingredient of possible) {
        if (!recipe.ingredients.has(ingredient)) {
          disqualified.add(ingredient);
        }
      }
    }
    matches[allergen] = [...possible].filter((a) => !disqualified.has(a));
  }

  do {
    let definite = Object.values(matches).reduce((acc, curr) => {
      if (curr.length === 1) {
        return [...acc, ...curr];
      }
      return acc;
    }, []);

    matches = Object.entries(matches).reduce((acc, [key, val]) => {
      if (val.length > 1) {
        acc[key] = val.filter((el) => !definite.includes(el));
      }
      return acc;
    }, matches);
  } while (Object.values(matches).flat().length > allergens.size);

  return Object.entries(matches).reduce(
    (acc, [key, val]) => ({ ...acc, [key]: val[0] }),
    {}
  );
}

function orderAlphabetically(matches) {
  const ordered = Object.entries(matches)
    .sort((a, b) => (a[0] < b[0] ? -1 : 1))
    .map((el) => el[1])
    .join(",");
  return ordered;
}

function partOne() {
  const recipes = createRecipes();
  const ingredients = getNonAllergenIngredients(recipes);
  const count = createCounter("ingredients");
  return [...ingredients].reduce((acc, curr) => {
    const num = count(recipes, curr);
    return acc + num;
  }, 0);
}

function partTwo() {
  const recipes = createRecipes();
  const inerts = getNonAllergenIngredients(recipes);
  const allergenMap = getIngredientAllergens(recipes, inerts);
  return orderAlphabetically(allergenMap);
}

console.log("Part 1: ", partOne());
console.log("Part 2: ", partTwo());
