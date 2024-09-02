-- CreateEnum
CREATE TYPE "EntryType" AS ENUM ('Reaction', 'Button');

-- CreateTable
CREATE TABLE "Guild" (
    "id" BIGINT NOT NULL,
    "prefix" TEXT NOT NULL DEFAULT '!',
    "disabled_commands" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "entry_type" "EntryType" NOT NULL DEFAULT 'Button',
    "creator_roles" TEXT[] DEFAULT ARRAY[]::TEXT[],
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,
    "end_embed_settings_id" TEXT NOT NULL,
    "start_embed_settings_id" TEXT NOT NULL,

    CONSTRAINT "Guild_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "EmbedSettings" (
    "id" TEXT NOT NULL,
    "color" TEXT NOT NULL DEFAULT '0x000000',
    "title" TEXT NOT NULL DEFAULT '{{prize}}',
    "description" TEXT NOT NULL DEFAULT '{{description}}',
    "image" TEXT NOT NULL DEFAULT '',
    "thumbnail" TEXT NOT NULL DEFAULT '',
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "EmbedSettings_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Giveaway" (
    "message_id" BIGINT NOT NULL,
    "guild_id" BIGINT NOT NULL,
    "channel_id" BIGINT NOT NULL,
    "prize" TEXT NOT NULL,
    "host" TEXT NOT NULL,
    "start_at" TIMESTAMP(3) NOT NULL,
    "end_at" TIMESTAMP(3) NOT NULL,
    "winners" INTEGER NOT NULL,
    "is_ended" BOOLEAN NOT NULL DEFAULT false,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "Giveaway_pkey" PRIMARY KEY ("message_id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Guild_end_embed_settings_id_key" ON "Guild"("end_embed_settings_id");

-- CreateIndex
CREATE UNIQUE INDEX "Guild_start_embed_settings_id_key" ON "Guild"("start_embed_settings_id");

-- AddForeignKey
ALTER TABLE "Guild" ADD CONSTRAINT "Guild_start_embed_settings_id_fkey" FOREIGN KEY ("start_embed_settings_id") REFERENCES "EmbedSettings"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Guild" ADD CONSTRAINT "Guild_end_embed_settings_id_fkey" FOREIGN KEY ("end_embed_settings_id") REFERENCES "EmbedSettings"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Giveaway" ADD CONSTRAINT "Giveaway_guild_id_fkey" FOREIGN KEY ("guild_id") REFERENCES "Guild"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
