/*
  Warnings:

  - The primary key for the `Giveaway` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `Guild` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - Changed the type of `guild_id` on the `Giveaway` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `channel_id` on the `Giveaway` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `message_id` on the `Giveaway` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.
  - Changed the type of `id` on the `Guild` table. No cast exists, the column would be dropped and recreated, which cannot be done if there is data, since the column is required.

*/
-- DropForeignKey
ALTER TABLE "Giveaway" DROP CONSTRAINT "Giveaway_guild_id_fkey";

-- AlterTable
ALTER TABLE "Giveaway" DROP CONSTRAINT "Giveaway_pkey",
DROP COLUMN "guild_id",
ADD COLUMN     "guild_id" BIGINT NOT NULL,
DROP COLUMN "channel_id",
ADD COLUMN     "channel_id" BIGINT NOT NULL,
DROP COLUMN "message_id",
ADD COLUMN     "message_id" BIGINT NOT NULL,
ADD CONSTRAINT "Giveaway_pkey" PRIMARY KEY ("message_id");

-- AlterTable
ALTER TABLE "Guild" DROP CONSTRAINT "Guild_pkey",
DROP COLUMN "id",
ADD COLUMN     "id" BIGINT NOT NULL,
ADD CONSTRAINT "Guild_pkey" PRIMARY KEY ("id");

-- AddForeignKey
ALTER TABLE "Giveaway" ADD CONSTRAINT "Giveaway_guild_id_fkey" FOREIGN KEY ("guild_id") REFERENCES "Guild"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
