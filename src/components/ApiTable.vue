<template>
  <table>
    <thead>
      <tr>
        <th>列1</th>
        <th>列2</th>
        <th>列3</th>
        <th>列4</th>
        <th>列5</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <th>列11</th>
        <th>列12</th>
        <th>列13</th>
        <th>列14</th>
        <th>列15</th>
      </tr>
      <tr>
        <th>列112312323213211</th>
        <th>列12</th>
        <th>列13</th>
        <th>列112312323213211</th>
        <th>列15</th>
      </tr>
      <tr>
        <th>列11</th>
        <th>列12</th>
        <th>列13</th>
        <th>列14</th>
        <th>列15</th>
      </tr>
    </tbody>
  </table>
</template>

<style scoped>
table {
  width: 100%;
  border-collapse: collapse;
}

th,
td {
  border: 1px solid #999;
  text-align: center;
  padding: 10px 0;
}

table thead tr {
  background-color: #008c8c;
  color: #fff;
}

table tbody tr:nth-child(odd) {
  background-color: #eee;
}

table tbody tr:hover {
  background-color: #ccc;
}

table tbody tr td:first-child {
  color: #f40;
}
</style>
