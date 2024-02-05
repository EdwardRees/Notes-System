//import { integer, pgEnum, pgTable, , uniqueIndex, varchar } from 'drizzle-orm/pg-core';
import { uuid, text, pgTable } from 'drizzle-orm/pg-core';

// declaring enum in database
const notes = pgTable('notes', {
  id: uuid('id').defaultRandom().primaryKey(),
  user_id: uuid('user_id'),
  title: text('title'),
  description: text('description'),
  categories: text('categories'),
  tags: text('tags')
});

export default notes;
