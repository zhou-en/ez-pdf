import { z } from 'zod';

export const credentialsSchema = z.object({
  email: z.string().email('Enter a valid email address.'),
  password: z.string().min(8, 'Use at least 8 characters.'),
});

export const signUpSchema = credentialsSchema.extend({
  name: z.string().trim().min(1, 'Enter your name.').max(80),
});

export type Credentials = z.infer<typeof credentialsSchema>;
export type SignUp = z.infer<typeof signUpSchema>;
