/*
  Warnings:

  - The primary key for the `Giveaway` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `id` on the `Giveaway` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "Giveaway" DROP CONSTRAINT "Giveaway_pkey",
DROP COLUMN "id",
ADD CONSTRAINT "Giveaway_pkey" PRIMARY KEY ("message_id");
