using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Linq.Expressions;
using System.Runtime.CompilerServices;
using System.Text;
using System.Threading.Tasks;

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

                        // find euclidian distance between head and tail and update the tail's location
                        double euclid = EuclidDistance(head[0], head[1], tail[0], tail[1]);

                        if (euclid > MAN_THRESH)
                        {
                            Console.WriteLine("Threshold Exceeded, Euclid Distance: " + euclid);
                            int[] newTail = new int[] { tail[0], tail[1] };
                            for (int j = 0; j < 8; j++)
                            {
                                double min = euclid;
                                int[] delta = MapGrid(j);
                                int newX = (tail[0] + delta[0]);
                                int newY = (tail[1] + delta[1]);
                                double newDistance = EuclidDistance(head[0], head[1], (tail[0] + delta[0]), (tail[1]+ delta[1]));

                                if (newDistance < min)
                                {
                                    min = newDistance;
                                    newTail = delta;
                                }
                            }

                            tail = newTail;
                        }
                        else
                            Console.WriteLine("No movement required.");
                    }
                }
            }
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
                    return new int[] { -1, -1 };
                case 1:
                    return new int[] { -1, 0 };
                case 2:
                    return new int[] { -1, 1 };
                case 3:
                    return new int[] { -1, 0 };
                case 4:
                    return new int[] { 0, 0 };
                case 5:
                    return new int[] { 1, 0 };
                case 6:
                    return new int[] { 1, 0 };
                case 7:
                    return new int[] { 1, 1 };

                default:
                    return new int[2];
            }

        }
    }
}
