-- AlterTable
ALTER TABLE "EmbedSettings" ALTER COLUMN "description" SET DEFAULT 'Prize: {{prize}}
Entries: {{entries_count}}
Winners: {{winners}}
Time: {{timer}} {{ends_at}}';

-- AlterTable
ALTER TABLE "Guild" ADD COLUMN     "reaction" TEXT NOT NULL DEFAULT '';
