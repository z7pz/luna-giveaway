/*
  Warnings:

  - You are about to drop the column `type` on the `EmbedSettings` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "EmbedSettings" DROP COLUMN "type";

-- DropEnum
DROP TYPE "EmbedType";
