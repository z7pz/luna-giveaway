-- CreateTable
CREATE TABLE "Guild" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "prefix" TEXT NOT NULL,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME NOT NULL,
    "end_embed_settings_id" TEXT NOT NULL,
    "start_embed_settings_id" TEXT NOT NULL,
    CONSTRAINT "Guild_start_embed_settings_id_fkey" FOREIGN KEY ("start_embed_settings_id") REFERENCES "EmbedSettings" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "Guild_end_embed_settings_id_fkey" FOREIGN KEY ("end_embed_settings_id") REFERENCES "EmbedSettings" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "EmbedSettings" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "color" TEXT NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "image" TEXT NOT NULL,
    "thumbnail" TEXT NOT NULL,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Guild_end_embed_settings_id_key" ON "Guild"("end_embed_settings_id");

-- CreateIndex
CREATE UNIQUE INDEX "Guild_start_embed_settings_id_key" ON "Guild"("start_embed_settings_id");
