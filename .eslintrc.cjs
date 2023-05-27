/** @type {import("eslint").Linter.Config} */
const config = {
  extends: [
    // add more generic rulesets here, such as:
    // 'eslint:recommended',
    'standard-with-typescript',
    'plugin:vue/vue3-recommended'
    // 'plugin:vue/recommended' // Use this if you are using Vue.js 2.x.
  ],
  parser: 'vue-eslint-parser',
  parserOptions: {
    parser: '@typescript-eslint/parser',
    sourceType: 'module',
    project: './tsconfig.json',
    extraFileExtensions: ['.vue']
  },
  rules: {
    // override/add rules settings here, such as:
    // 'vue/no-unused-vars': 'error'
  }
}

module.exports = config
