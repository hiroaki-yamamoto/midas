db.createUser({
  user: "midas",
  pwd: "midas",
  mechanisms: ["SCRAM-SHA-256"],
  roles: [
    { role: "readWrite", db: "midas" },
    { role: "read", db: "admin" },
  ]
});
