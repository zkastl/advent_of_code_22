using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Linq.Expressions;
using System.Runtime.CompilerServices;
using System.Security.Principal;
using System.Text;
using System.Threading.Tasks;
using System.Windows;

namespace Aoc2022.cs_src
{
    public class Day9
    {
        public static void Puzzle()
        {
            double MAN_THRESH = Math.Sqrt(2);
            

            using (StreamReader sr = new StreamReader(new FileStream(Environment.CurrentDirectory + "/day9.txt", FileMode.Open)))
            {
                if (sr == null)
                    return;

                int[] head = new int[2];
                int[] tail = new int[2];
                var visited = new HashSet<string>();
                visited.Add(tail[0] + "," + tail[1]);

                string input = sr.ReadToEnd();
                AIMain(input);
                sr.BaseStream.Position = 0;

                return;

                // Read in a line and perform its actions
                while (!sr.EndOfStream)
                {

                    string[] tokens = sr.ReadLine().Split(' ');

                    for (int i = 0; i < uint.Parse(tokens[1]); i++)
                    {
                        // move the head in the appropiate direction
                        switch (tokens[0])
                        {
                            case "L":
                                head[0]--;
                                break;

                            case "U":
                                head[1]++;
                                break;

                            case "R":
                                head[0]++;
                                break;

                            case "D":
                                head[1]--;
                                break;

                            default:
                                throw new Exception("CANNOT PARSE LINE");
                        }

                        Console.WriteLine("Head location: (" + head[0] + "," + head[1] + ")");
                        // find euclidian distance between head and tail and update the tail's location
                        double euclid = EuclidDistance(head[0], head[1], tail[0], tail[1]);

                        if (euclid > MAN_THRESH)
                        {
                            //Console.WriteLine("Threshold Exceeded, Euclid Distance: " + euclid);
                            int[] newTail = new int[] { tail[0], tail[1] };
                            double min = euclid;
                            for (int j = 0; j < 8; j++)
                            {
                                int[] delta = MapGrid(j);
                                int newX = (tail[0] + delta[0]);
                                int newY = (tail[1] + delta[1]);
                                double newDistance = EuclidDistance(head[0], head[1], newX, newY);

                                if (newDistance < min)
                                {
                                    min = newDistance;
                                    newTail = new int[] { newX, newY };
                                }
                            }

                            tail = newTail;
                            visited.Add(tail[0] + "," + tail[1]);
                        }
                        Console.WriteLine("Tail location: (" + tail[0] + "," + tail[1] + ")");
                    }

                    //PrintGrid(visited);
                }

                Console.WriteLine("Number of unique locations visited: " + visited.Count);
            }
        }

        static void PrintGrid(int[][] grid)
        {

        }

        static double EuclidDistance(int p1, int p2, int q1, int q2)
        {
            return Math.Sqrt(
                Math.Pow(q1 - p1, 2) +
                Math.Pow(q2 - p2, 2)
                );
        }

        static int[] MapGrid(int pos)
        {
            switch (pos)
            {
                case 0:
                    return new int[] { -1, 1 };
                case 1:
                    return new int[] { 0, 1 };
                case 2:
                    return new int[] { 1, 1 };
                case 3:
                    return new int[] { -1, 0 };
                case 4:
                    return new int[] { 0, 0 };
                case 5:
                    return new int[] { 1, 0 };
                case 6:
                    return new int[] { -1, -1 };
                case 7:
                    return new int[] { 0, -1 };
                case 8:
                    return new int[] { 1, -1 };

                default:
                    return new int[2];
            }

        }

        static void AIMain(string textInput)
        {
            // Initialize the head and tail positions
            int headX = 0;
            int headY = 0;
            int tailX = 0;
            int tailY = 0;

            // Read the input and store the movements in a list
            List<Tuple<char, int>> movements = new List<Tuple<char, int>>();
            string input = textInput;
            string[] lines = input.Split('\n');
            foreach (string line in lines)
            {
                char direction = line[0];
                int distance = int.Parse(line.Substring(2));
                movements.Add(new Tuple<char, int>(direction, distance));
            }

            // Iterate through the movements and update the positions of the head and tail
            foreach (Tuple<char, int> movement in movements)
            {
                // Update the position of the head
                switch (movement.Item1)
                {
                    case 'U':
                        headY -= movement.Item2;
                        break;
                    case 'D':
                        headY += movement.Item2;
                        break;
                    case 'L':
                        headX -= movement.Item2;
                        break;
                    case 'R':
                        headX += movement.Item2;
                        break;
                }

                // Update the position of the tail
                int dx = headX - tailX;
                int dy = headY - tailY;
                if (dx == 0 && dy == 0)
                {
                    // Do nothing, the head and tail are already overlapping
                }
                else if (Math.Abs(dx) == 1 && Math.Abs(dy) == 1)
                {
                    // The head and tail are diagonally adjacent, so do nothing
                }
                else if (Math.Abs(dx) <= 2 && Math.Abs(dy) <= 2)
                {
                    // The head and tail are two steps apart in a straight line, so the tail moves one step in the same direction
                    tailX += dx / Math.Abs(dx);
                    tailY += dy / Math.Abs(dy);
                }
                else
                {
                    // The head and tail are not touching, so the tail moves one step diagonally
                    tailX += dx / Math.Abs(dx);
                    tailY += dy / Math.Abs(dy);
                }
            }

            // Print the final position of the head and tail
            Console.WriteLine($"Head: ({headX}, {headY})");
            Console.WriteLine($"Tail: ({tailX}, {tailY})");
        }
    }
}
